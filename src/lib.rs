use std::iter::FromIterator;

#[derive(Eq, PartialEq, Debug, Ord, PartialOrd, Clone, Copy, Hash)]
pub enum Validation<T, E> {
    Ok(T),
    Err(E),
}

impl<T, E> Validation<T, E> {
    pub fn into_result(self) -> Result<T, E> {
        match self {
            Validation::Ok(t) => Ok(t),
            Validation::Err(e) => Err(e),
        }
    }
}

struct Phase1<E, I> {
    iter: I,
    err: Option<E>,
}

impl<T, E, I: Iterator<Item = Result<T, E>>> Iterator for &mut Phase1<E, I> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        assert!(self.err.is_none());
        match self.iter.next()? {
            Ok(t) => Some(t),
            Err(e) => {
                self.err = Some(e);
                None
            }
        }
    }
}

struct Phase2<E, I> {
    first: Option<E>,
    iter: I,
}

impl<T, E, I: Iterator<Item = Result<T, E>>> Iterator for Phase2<E, I> {
    type Item = E;

    fn next(&mut self) -> Option<E> {
        match self.first.take() {
            Some(e) => Some(e),
            None => loop {
                match self.iter.next()? {
                    Ok(_) => (),
                    Err(e) => break Some(e),
                }
            },
        }
    }
}

impl<T, VT: FromIterator<T>, E, VE: FromIterator<E>> FromIterator<Result<T, E>>
    for Validation<VT, VE>
{
    fn from_iter<I: IntoIterator<Item = Result<T, E>>>(iter: I) -> Self {
        let mut phase1 = Phase1 {
            err: None,
            iter: iter.into_iter(),
        };
        let vt = (&mut phase1).collect();
        match phase1.err {
            None => Validation::Ok(vt),
            Some(e) => {
                let phase2 = Phase2 {
                    first: Some(e),
                    iter: phase1.iter,
                };
                Validation::Err(phase2.collect())
            }
        }
    }
}

impl<T, VT: FromIterator<T>, E, VE: FromIterator<E>> FromIterator<Validation<T, E>>
    for Validation<VT, VE>
{
    fn from_iter<I: IntoIterator<Item = Validation<T, E>>>(iter: I) -> Self {
        iter.into_iter().map(|v| v.into_result()).collect()
    }
}

// FIXME impl Try for Validation

#[cfg(test)]
mod tests {
    #[derive(Eq, PartialEq, Debug)]
    struct Good(i32);
    #[derive(Eq, PartialEq, Debug)]
    struct Bad(i32);
    use super::Validation;

    #[test]
    fn no_results_is_ok() {
        let input: Vec<Result<Good, Bad>> = vec![];
        let res: Validation<Vec<Good>, Vec<Bad>> = input.into_iter().collect();
        assert_eq!(res, Validation::Ok(vec![]));
        assert_eq!(res.into_result(), Ok(vec![]));
    }

    #[test]
    fn one_good() {
        let input: Vec<Result<Good, Bad>> = vec![Ok(Good(1))];
        let res: Validation<Vec<Good>, Vec<Bad>> = input.into_iter().collect();
        assert_eq!(res, Validation::Ok(vec![Good(1)]));
    }

    #[test]
    fn one_bad() {
        let input: Vec<Result<Good, Bad>> = vec![Err(Bad(1))];
        let res: Validation<Vec<Good>, Vec<Bad>> = input.into_iter().collect();
        assert_eq!(res, Validation::Err(vec![Bad(1)]));
    }

    #[test]
    fn one_good_one_bad() {
        let input: Vec<Result<Good, Bad>> = vec![Err(Bad(1)), Ok(Good(2))];
        let res: Validation<Vec<Good>, Vec<Bad>> = input.into_iter().collect();
        assert_eq!(res, Validation::Err(vec![Bad(1)]));
    }

    #[test]
    fn all_good() {
        let input: Vec<Result<Good, Bad>> = (0..10).map(|x| Ok(Good(x))).collect();
        let res: Validation<Vec<Good>, Vec<Bad>> = input.into_iter().collect();
        let expected = (0..10).map(|x| Validation::Ok(Good(x))).collect();
        assert_eq!(res, expected);
    }

    #[test]
    fn all_bad() {
        let input: Vec<Result<Good, Bad>> = (0..10).map(|x| Err(Bad(x))).collect();
        let res: Validation<Vec<Good>, Vec<Bad>> = input.into_iter().collect();
        let expected = (0..10).map(|x| Validation::Err(Bad(x))).collect();
        assert_eq!(res, expected);
    }
}
