use std::rc::Rc;

use ndarray::{s, Array1, Array2};

use super::knn::KNN;
use crate::{interpolate::Input, metric::Metric};

pub struct Commons<Point, Finder>
where
    Point: Metric,
    Finder: KNN<Point = Point>,
{
    pub(crate) points: Vec<Rc<Point>>,
    pub(crate) values: Vec<f64>,
    pub(crate) finder: Option<Finder>,
}

impl<Point, Finder> Commons<Point, Finder>
where
    Point: Metric,
    Finder: KNN<Point = Point>,
{
    pub fn new(inputs: Vec<Input<Point>>) -> Self {
        let values = inputs.iter().map(|i| i.value).collect();
        let points = inputs.into_iter().map(|i| Rc::new(i.point)).collect();

        Self {
            points,
            values,
            finder: None,
        }
    }

    pub fn set_finder(&mut self, finder: Finder) {
        self.finder = Some(finder)
    }
}

fn split_2d(points: Array2<f64>) -> (Vec<Array1<f64>>, Vec<f64>) {
    let values = points.outer_iter().map(|ar| ar[ar.len() - 1]).collect();
    let points = points
        .slice(s![.., ..-1])
        .outer_iter()
        .map(|ar| ar.to_owned())
        .collect();

    (points, values)
}

impl<Finder> From<Array2<f64>> for Commons<Array1<f64>, Finder>
where
    Finder: KNN<Point = Array1<f64>>,
{
    fn from(inputs: Array2<f64>) -> Self {
        let (points, values) = split_2d(inputs);
        Self::new(Input::stack(points, values))
    }
}
