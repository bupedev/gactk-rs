use std::fmt::{Display, Error, Formatter};

pub enum VertexType {
    Corner(usize),
    Centre(usize),
    Edge(usize),
}

pub enum TransformationSource {
    Origin(Option<usize>),
    Vertex(VertexType),
}

pub enum Transformation {
    Rotation(TransformationSource),
    Reflection(TransformationSource),
}

pub struct Configuration {
    polygon_placement: Vec<Vec<usize>>,
    transformations: Vec<Transformation>,
}

impl Display for VertexType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            VertexType::Corner(index) => write!(f, "v{}", index),
            VertexType::Centre(index) => write!(f, "c{}", index),
            VertexType::Edge(index) => write!(f, "h{}", index),
        }
    }
}

impl Display for TransformationSource {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            TransformationSource::Origin(angle_option) => {
                if let Some(angle) = angle_option {
                    write!(f, "{}", angle)
                } else {
                    write!(f, "")
                }
            }
            TransformationSource::Vertex(vertex_type) => write!(f, "({})", vertex_type),
        }
    }
}

impl Display for Transformation {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            Transformation::Rotation(source) => write!(f, "r{}", source.to_string()),
            Transformation::Reflection(source) => write!(f, "m{}", source.to_string()),
        }
    }
}

impl Display for Configuration {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let mut formatted = "".to_string();
        formatted.push_str(
            self.polygon_placement
                .iter()
                .map(|phase| {
                    phase
                        .iter()
                        .map(|side| side.to_string())
                        .collect::<Vec<_>>()
                        .join(",")
                })
                .collect::<Vec<_>>()
                .join("-")
                .as_str(),
        );
        formatted.push('/');
        formatted.push_str(
            self.transformations
                .iter()
                .map(|t| t.to_string())
                .collect::<Vec<_>>()
                .join("/")
                .as_str()
        );
        write!(f, "{}", formatted)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    mod configuration {
        use super::*;

        #[test]
        fn display() { 
            fn test(config: Configuration, expected: &str) {
                let actual = config.to_string();
                assert_eq!(actual, expected);
            }

            test(Configuration { 
                polygon_placement: vec![vec![3]], 
                transformations: vec![
                    Transformation::Reflection(TransformationSource::Origin(Some(30))),
                    Transformation::Rotation(TransformationSource::Vertex(VertexType::Edge(2)))
                ] 
            }, "3/m30/r(h2)");
            
            test(Configuration { 
                polygon_placement: vec![vec![12], vec![6, 4]], 
                transformations: vec![
                    Transformation::Reflection(TransformationSource::Origin(Some(30))),
                    Transformation::Rotation(TransformationSource::Vertex(VertexType::Centre(2)))
                ] 
            }, "12-6,4/m30/r(c2)");
            
            test(Configuration { 
                polygon_placement: vec![vec![6],vec![3, 0, 3, 3, 3, 3]], 
                transformations: vec![
                    Transformation::Rotation(TransformationSource::Vertex(VertexType::Edge(4))),
                    Transformation::Rotation(TransformationSource::Vertex(VertexType::Corner(15))),
                    Transformation::Rotation(TransformationSource::Vertex(VertexType::Corner(30))),
                ] 
            }, "6-3,0,3,3,3,3/r(h4)/r(v15)/r(v30)");
        }
    }
}