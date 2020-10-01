use std::iter::FromIterator;

#[derive(Eq, PartialEq, Debug, Ord, PartialOrd, Clone, Copy, Hash)]
pub enum Validation<T, E> {
    Ok(T),
    Err(E),
}

impl<T, E> Validation<T, E> {
    pub fn to_result(self) -> Result<T, E> {
        match self {
            Validation::Ok(t) => Ok(t),
            Validation::Err(e) => Err(e),
        }
    }
}

// helper enum
enum EVec<T, E> { VOk(Vec<T>), VErr(Vec<E>) }
impl<T, E> FromIterator<Result<T, E>> for EVec<T, E> {
    fn from_iter<I: IntoIterator<Item = Result<T, E>>>(iter: I) -> Self {
        use EVec::*;
        let mut res = VOk(vec![]);
        for x in iter {
            match x {
                Ok(t) => {
                    match &mut res {
                        VOk(v) => v.push(t),
                        VErr(_) => (),
                    }
                }
                Err(e) => {
                    res = match res {
                        VOk(_) => VErr(vec![e]),
                        VErr(mut v) => {
                            v.push(e);
                            VErr(v)
                        }
                    };
                }
            }
        }
        res
    }
}

impl<T, VT: FromIterator<T>, E, VE: FromIterator<E>> FromIterator<Result<T, E>> for Validation<VT, VE> {
    fn from_iter<I: IntoIterator<Item = Result<T, E>>>(iter: I) -> Self {
        let evec: EVec<T, E> = iter.into_iter().collect();
        match evec {
            EVec::VOk(vok) => Validation::Ok(vok.into_iter().collect()),
            EVec::VErr(verr) => Validation::Err(verr.into_iter().collect()),
        }
    }
}

impl<T, VT: FromIterator<T>, E, VE: FromIterator<E>> FromIterator<Validation<T, E>> for Validation<VT, VE> {
    fn from_iter<I: IntoIterator<Item = Validation<T, E>>>(iter: I) -> Self {
        iter.into_iter().map(|v| v.to_result()).collect()
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
        assert_eq!(res.to_result(), Ok(vec![]));
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
