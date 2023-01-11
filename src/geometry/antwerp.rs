use std::{fmt::{Display, Error, Formatter}, collections::VecDeque};

#[derive(Debug, PartialEq)]
pub enum VertexType {
    Corner(usize),
    Centre(usize),
    Edge(usize),
}

#[derive(Debug, PartialEq)]
pub enum TransformationSource {
    Origin(Option<usize>),
    Vertex(VertexType),
}

#[derive(Debug, PartialEq)]
pub enum Transformation {
    Rotation(TransformationSource),
    Reflection(TransformationSource),
}

#[derive(Debug, PartialEq)]
pub struct Configuration {
    phases: Vec<Vec<usize>>,
    transformations: Vec<Transformation>,
}

impl Configuration {
    fn new(phases: Vec<Vec<usize>>, transformations: Vec<Transformation>) -> Self {
        Self {
            phases,
            transformations,
        }
    }
}

impl TryFrom<&str> for Configuration {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut command_strings: VecDeque<&str> = value.split("/").collect();
        if command_strings.len() < 2 {
            panic!("Configuration string must have at least one transformation");
        }

        let mut phases: Vec<Vec<usize>> = vec![];
        let phases_strings: Vec<&str> = command_strings.pop_front().unwrap().split("-").collect();
        for phase_string in phases_strings {
            let mut phase = vec![];
            let shape_strings = phase_string.split(",");
            for shape_string in shape_strings {
                let shape_parse_result = shape_string.parse::<usize>();
                if let Ok(shape) = shape_parse_result {
                    phase.push(shape);
                } else {
                    panic!("Invalid shape in configuration string");
                }
            }
            phases.push(phase)
        }

        let mut transformations = vec![];
        for transformation_string in command_strings {
            let mut transformation_string_chars = transformation_string.chars();
            let transformation_type_char_result = transformation_string_chars.next();
            if let Some(transformation_type_char) = transformation_type_char_result {
                let transformation_source_string: String = transformation_string_chars.collect();
                if transformation_source_string.len() == 0 {
                    let transformation = match transformation_type_char {
                        'r' => Transformation::Rotation(TransformationSource::Origin(None)),
                        'm' => Transformation::Reflection(TransformationSource::Origin(None)),
                        _ => return Err("Unknown transformation character in configuration string")   
                    };
                    transformations.push(transformation);
                }
                else {
                    let transformation_source_numeric_parse_result = transformation_source_string.parse::<usize>();
                    if let Ok(transformation_source_numeric) = transformation_source_numeric_parse_result {
                        let transformation = match transformation_type_char {
                            'r' => Transformation::Rotation(TransformationSource::Origin(Some(transformation_source_numeric))),
                            'm' => Transformation::Reflection(TransformationSource::Origin(Some(transformation_source_numeric))),
                            _ => return Err("Unknown transformation character in configuration string")   
                        };
                        transformations.push(transformation);
                    } else {
                        let mut chars = transformation_source_string.chars();
                        chars.next();
                        chars.next_back();
                        let vertex_string = chars.as_str();
                        let mut vertex_string_chars = vertex_string.chars();
                        let vertex_type_char_result = vertex_string_chars.next();
                        if let Some(vertex_type_char) = vertex_type_char_result {
                            let vertex_index_string: String = vertex_string_chars.collect();
                            let vertex_index_numeric_parse_result = vertex_index_string.parse::<usize>();
                            if let Ok(vertex_index_numeric_parse_result) = vertex_index_numeric_parse_result {
                                let vertex_type = match vertex_type_char {
                                    'v' => VertexType::Corner(vertex_index_numeric_parse_result),
                                    'c' => VertexType::Centre(vertex_index_numeric_parse_result),
                                    'h' => VertexType::Edge(vertex_index_numeric_parse_result),
                                    _ => return Err("Unknown vertex type character in configuration string")   
                                };
                                let transformation = match transformation_type_char {
                                    'r' => Transformation::Rotation(TransformationSource::Vertex(vertex_type)),
                                    'm' => Transformation::Reflection(TransformationSource::Vertex(vertex_type)),
                                    _ => return Err("Unknown transformation character in configuration string")   
                                };
                                transformations.push(transformation);
                            }
                            else {
                                return Err("Invalid vertex index in configuration string")
                            }
                        }
                        else {
                            return Err("Empty vertex specifier in configuration string")
                        }
                    }
                }
            }
            else {
                return Err("Empty transformation in configuration string")
            }
        }

        Ok(Configuration::new(phases, transformations))
    }
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
            self.phases
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
                .as_str(),
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

            test(
                Configuration {
                    phases: vec![vec![3]],
                    transformations: vec![
                        Transformation::Reflection(TransformationSource::Origin(Some(30))),
                        Transformation::Rotation(TransformationSource::Vertex(VertexType::Edge(2))),
                    ],
                },
                "3/m30/r(h2)",
            );

            test(
                Configuration {
                    phases: vec![vec![12], vec![6, 4]],
                    transformations: vec![
                        Transformation::Reflection(TransformationSource::Origin(Some(30))),
                        Transformation::Rotation(TransformationSource::Vertex(VertexType::Centre(
                            2,
                        ))),
                    ],
                },
                "12-6,4/m30/r(c2)",
            );

            test(
                Configuration {
                    phases: vec![vec![6], vec![3, 0, 3, 3, 3, 3]],
                    transformations: vec![
                        Transformation::Rotation(TransformationSource::Vertex(VertexType::Edge(4))),
                        Transformation::Rotation(TransformationSource::Vertex(VertexType::Corner(
                            15,
                        ))),
                        Transformation::Rotation(TransformationSource::Vertex(VertexType::Corner(
                            30,
                        ))),
                    ],
                },
                "6-3,0,3,3,3,3/r(h4)/r(v15)/r(v30)",
            );
        }

        #[test]
        fn string_conversion() {
            fn test(config_string: &str, expected: Result<Configuration, Error>) {
                let actual = Configuration::try_from(config_string);
                assert_eq!(actual, expected);
            }

            test(
                "3/m30/r(h2)",
                Ok(Configuration {
                    phases: vec![vec![3]],
                    transformations: vec![
                        Transformation::Reflection(TransformationSource::Origin(Some(30))),
                        Transformation::Rotation(TransformationSource::Vertex(VertexType::Edge(2))),
                    ],
                })
            );

            test(
                "12-6,4/m30/r(c2)",
                Ok(Configuration {
                    phases: vec![vec![12], vec![6, 4]],
                    transformations: vec![
                        Transformation::Reflection(TransformationSource::Origin(Some(30))),
                        Transformation::Rotation(TransformationSource::Vertex(VertexType::Centre(
                            2,
                        ))),
                    ],
                })
            );

            test(
                "6-3,0,3,3,3,3/r(h4)/r(v15)/r(v30)",
                Ok(Configuration {
                    phases: vec![vec![6], vec![3, 0, 3, 3, 3, 3]],
                    transformations: vec![
                        Transformation::Rotation(TransformationSource::Vertex(VertexType::Edge(4))),
                        Transformation::Rotation(TransformationSource::Vertex(VertexType::Corner(
                            15,
                        ))),
                        Transformation::Rotation(TransformationSource::Vertex(VertexType::Corner(
                            30,
                        ))),
                    ],
                })
            );
        }
    }
}
