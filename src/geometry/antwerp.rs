use std::{
    collections::VecDeque,
    fmt::{Display, Error, Formatter},
};

use num_traits::{real::Real, Euclid};

use crate::numerics::RealConst;

use super::{Poly2, Vec2};

#[derive(Debug, PartialEq)]
pub enum VertexType {
    Corner(usize),
    Centre(usize),
    Edge(usize),
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

#[derive(Debug, PartialEq)]
pub enum TransformationSource {
    Origin(Option<usize>),
    Vertex(VertexType),
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

#[derive(Debug, PartialEq)]
pub enum Transformation {
    Rotation(TransformationSource),
    Reflection(TransformationSource),
}

impl Display for Transformation {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            Transformation::Rotation(source) => write!(f, "r{}", source.to_string()),
            Transformation::Reflection(source) => write!(f, "m{}", source.to_string()),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Configuration {
    pub seed: usize,
    pub phases: Vec<Vec<usize>>,
    pub transformations: Vec<Transformation>,
}

impl Configuration {
    fn new(seed: usize, phases: Vec<Vec<usize>>, transformations: Vec<Transformation>) -> Self {
        Self {
            seed,
            phases,
            transformations,
        }
    }
}

impl Display for Configuration {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let mut formatted = self.seed.to_string();
        if self.phases.len() > 0 {
            formatted.push('-');
        }
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

impl TryFrom<&str> for Configuration {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut command_strings: VecDeque<&str> = value.split("/").collect();
        if command_strings.len() < 2 {
            return Err("Configuration string must have at least one transformation");
        }

        let mut seed: usize = usize::MAX;
        let mut phases: Vec<Vec<usize>> = vec![];
        let phases_strings: Vec<&str> = command_strings.pop_front().unwrap().split("-").collect();
        for phase_string in phases_strings {
            if seed == usize::MAX {
                let phase_string_result = phase_string.parse::<usize>();
                if let Ok(seed_order) = phase_string_result {
                    seed = seed_order;
                } else {
                    return Err("Invalid seed in configuration string");
                }
            } else {
                let mut phase = vec![];
                let shape_strings = phase_string.split(",");
                for shape_string in shape_strings {
                    let shape_parse_result = shape_string.parse::<usize>();
                    if let Ok(shape) = shape_parse_result {
                        phase.push(shape);
                    } else {
                        return Err("Invalid shape in configuration string");
                    }
                }
                phases.push(phase)
            }
        }

        let mut transformations = vec![];
        for transformation_string in command_strings {
            let mut transformation_string_chars = transformation_string.chars();
            let transformation_type_char_result = transformation_string_chars.next();
            if let Some(transformation_type_char) = transformation_type_char_result {
                let mut push_transformation = |source| {
                    let transformation = match transformation_type_char {
                        'r' => Transformation::Rotation(source),
                        'm' => Transformation::Reflection(source),
                        _ => {
                            return Err("Unknown transformation character in configuration string")
                        }
                    };
                    transformations.push(transformation);
                    Ok(())
                };

                let transformation_source_string: String = transformation_string_chars.collect();
                if transformation_source_string.len() == 0 {
                    let result = push_transformation(TransformationSource::Origin(None));
                    if let Err(message) = result {
                        return Err(message);
                    }
                } else {
                    let transformation_source_numeric_parse_result =
                        transformation_source_string.parse::<usize>();
                    if let Ok(transformation_source_numeric) =
                        transformation_source_numeric_parse_result
                    {
                        let result = push_transformation(TransformationSource::Origin(Some(
                            transformation_source_numeric,
                        )));
                        if let Err(message) = result {
                            return Err(message);
                        }
                    } else {
                        let mut chars = transformation_source_string.chars();
                        chars.next();
                        chars.next_back();
                        let vertex_string = chars.as_str();
                        let mut vertex_string_chars = vertex_string.chars();
                        let vertex_type_char_result = vertex_string_chars.next();
                        if let Some(vertex_type_char) = vertex_type_char_result {
                            let vertex_index_string: String = vertex_string_chars.collect();
                            let vertex_index_numeric_parse_result =
                                vertex_index_string.parse::<usize>();
                            if let Ok(vertex_index_numeric_parse_result) =
                                vertex_index_numeric_parse_result
                            {
                                let vertex_type = match vertex_type_char {
                                    'v' => VertexType::Corner(vertex_index_numeric_parse_result),
                                    'c' => VertexType::Centre(vertex_index_numeric_parse_result),
                                    'h' => VertexType::Edge(vertex_index_numeric_parse_result),
                                    _ => {
                                        return Err(
                                            "Unknown vertex type character in configuration string",
                                        )
                                    }
                                };
                                let result =
                                    push_transformation(TransformationSource::Vertex(vertex_type));
                                if let Err(message) = result {
                                    return Err(message);
                                }
                            } else {
                                return Err("Invalid vertex index in configuration string");
                            }
                        } else {
                            return Err("Empty vertex specifier in configuration string");
                        }
                    }
                }
            } else {
                return Err("Empty transformation in configuration string");
            }
        }

        Ok(Configuration::new(seed, phases, transformations))
    }
}

#[derive(Debug, PartialEq)]
pub struct Lattice<T: Real> {
    pub tiles: Vec<Poly2<T>>,
    pub connectivity: Vec<Vec<usize>>,
}

impl<T: Real + RealConst + Euclid + Display> Lattice<T> {
    pub fn generate(config: &Configuration, iterations: usize) -> Self {
        let mut tiles: Vec<Poly2<T>> = vec![];
        let mut connectors: Vec<Vec2<T>> = vec![];
        
        for phase in &config.phases {
            for &shape_order in phase {
                let shape: Poly2<T> = create_tile(shape_order).expect("this is a poor error message");
                
                if tiles.len() == 0 {
                    tiles.push(shape)
                } else if tiles.len() == 1 {
                    let join_index = starting_index(shape_order);

                } else {

                }
            }
        }

        Self {
            tiles,
            connectivity: vec![],
        }
    }
}

fn create_seed_tile<T>(sides: usize) -> Result<Poly2<T>, &'static str>
where
    T: Real + RealConst + Euclid + Display,
{
    match create_tile::<T>(sides) {
        Ok(tile) => match sides {
            3 => Ok(tile.translate(Vec2::<T>::unit(T::FRAC_PI_3) * (T::HALF / T::FRAC_PI_3.sin()))),
            4 | 6 | 8 | 12 => Ok(tile.rotate(T::PI / T::from(sides).expect("cast failure"))),
            _ => Err("That shape isn't kosher fam..."),
        },
        Err(problem) => Err(problem),
    }
}

fn create_tile<T>(sides: usize) -> Result<Poly2<T>, &'static str>
where
    T: Real + RealConst + Euclid,
{
    match sides {
        3 | 4 | 6 | 8 | 12 => Ok(Poly2::regular(sides, T::one())),
        _ => Err("That shape isn't kosher fam..."),
    }
}

fn starting_index(sides: usize) -> Result<usize, &'static str> {
    match sides {
        3 | 4 | 6  => Ok(0),
        8 => Ok(1),
        12 => Ok(2),
        _ => Err("Invalid shape")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: mod vertex_type

    // TODO: mod transformation_source

    // TODO: mod transformation

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
                    seed: 3,
                    phases: vec![],
                    transformations: vec![
                        Transformation::Reflection(TransformationSource::Origin(Some(30))),
                        Transformation::Rotation(TransformationSource::Vertex(VertexType::Edge(2))),
                    ],
                },
                "3/m30/r(h2)",
            );

            test(
                Configuration {
                    seed: 12,
                    phases: vec![vec![6, 4]],
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
                    seed: 6,
                    phases: vec![vec![3, 0, 3, 3, 3, 3]],
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
            fn test(config_string: &str, expected: Result<Configuration, &str>) {
                let actual = Configuration::try_from(config_string);
                assert_eq!(actual, expected);
            }

            test(
                "3/m30/r(h2)",
                Ok(Configuration {
                    seed: 3,
                    phases: vec![],
                    transformations: vec![
                        Transformation::Reflection(TransformationSource::Origin(Some(30))),
                        Transformation::Rotation(TransformationSource::Vertex(VertexType::Edge(2))),
                    ],
                }),
            );

            test(
                "12-6,4/m30/r(c2)",
                Ok(Configuration {
                    seed: 12,
                    phases: vec![vec![6, 4]],
                    transformations: vec![
                        Transformation::Reflection(TransformationSource::Origin(Some(30))),
                        Transformation::Rotation(TransformationSource::Vertex(VertexType::Centre(
                            2,
                        ))),
                    ],
                }),
            );

            test(
                "6-3,0,3,3,3,3/r(h4)/r(v15)/r(v30)",
                Ok(Configuration {
                    seed: 6,
                    phases: vec![vec![3, 0, 3, 3, 3, 3]],
                    transformations: vec![
                        Transformation::Rotation(TransformationSource::Vertex(VertexType::Edge(4))),
                        Transformation::Rotation(TransformationSource::Vertex(VertexType::Corner(
                            15,
                        ))),
                        Transformation::Rotation(TransformationSource::Vertex(VertexType::Corner(
                            30,
                        ))),
                    ],
                }),
            );

            test(
                "3",
                Err("Configuration string must have at least one transformation"),
            );

            test("x/m30/r(h2)", Err("Invalid seed in configuration string"));

            test("3-x/m30/r(h2)", Err("Invalid shape in configuration string"));

            test(
                "3/x30/r(h2)",
                Err("Unknown transformation character in configuration string"),
            );

            test(
                "3/m30/r(x2)",
                Err("Unknown vertex type character in configuration string"),
            );
        }
    }

    mod lattice {}
}
