use nom::{
    character::complete::{alpha1, multispace0, multispace1, newline, not_line_ending, space0},
    combinator::opt,
    multi::many1,
    sequence::{delimited, preceded, terminated},
    IResult,
};

use nom_supreme::tag::complete::tag;

#[derive(Debug, PartialEq)]
struct Section {
    header: String,
    queries: Vec<String>,
    note: Option<String>,
}

#[derive(Debug, PartialEq)]
struct Plan {
    topic: String,
    sections: Vec<Section>,
}

fn parse_plan(input: &str) -> IResult<&str, Plan> {
    // Parse the Topic
    let (input, topic) = parse_topic(input)?;
    // Parse the sections
    let (input, sections) = parse_sections(input)?;

    Ok((input, Plan { topic, sections }))
}

fn parse_topic(input: &str) -> IResult<&str, String> {
    let (input, _) = tag("Topic:")(input)?;
    let (input, topic) = terminated(not_line_ending, newline)(input)?;
    Ok((input.trim(), topic.trim().to_string()))
}

fn parse_section(input: &str) -> IResult<&str, Section> {
    let (input, header) = parse_header(input)?;
    let (input, queries) = parse_queries(input)?;
    let (input, note) = parse_note(input)?;

    let (input, _) = multispace0(input)?;
    Ok((
        input,
        Section {
            header,
            queries,
            note,
        },
    ))
}

fn parse_sections(input: &str) -> IResult<&str, Vec<Section>> {
    let (input, _) = preceded(tag("Outline:"), multispace1)(input)?;
    let (input, sections) = many1(parse_section)(input)?;
    Ok((input, sections))
}

fn parse_header(input: &str) -> IResult<&str, String> {
    // Header is something like "I. Introduction"
    let (input, header) =
        delimited(preceded(alpha1, tag(". ")), not_line_ending, tag("\n"))(input)?;
    Ok((input, header.trim().to_string()))
}

fn parse_queries(input: &str) -> IResult<&str, Vec<String>> {
    // Queries are lines that start with "-"
    let (input, queries) = many1(parse_query)(input)?;
    Ok((input, queries))
}

fn parse_query(input: &str) -> IResult<&str, String> {
    let (input, _) = tag("- ")(input)?;
    let (input, query) = terminated(not_line_ending, multispace0)(input)?;
    Ok((input, query.trim().to_string()))
}

fn parse_note(input: &str) -> IResult<&str, Option<String>> {
    // Nota puede estar ausente, o bien puede estar al final de las queries
    let (input, note) = opt(preceded(
        multispace0,
        preceded(tag("Note: "), not_line_ending),
    ))(input)?;

    let (input, _) = multispace0(input)?; // Consume cualquier espacio restante
    let note = note.map(|s| s.to_string());
    Ok((input, note))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_topic_success() {
        let input = "Topic: The Impact of Social Media on Youth\n";
        let expected = "The Impact of Social Media on Youth".to_string();
        let result = parse_topic(input);
        assert!(result.is_ok());
        let (remaining, topic) = result.unwrap();
        assert_eq!(topic, expected);
        assert_eq!(remaining, "");
    }

    #[test]
    fn test_parse_topic_failure() {
        let input = "Topik: Missing correct keyword\n";
        let result = parse_topic(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_header_success() {
        let input = "II. Benefits of Social Media on Youth\n";
        let expected = "Benefits of Social Media on Youth".to_string();
        let result = parse_header(input);
        let (remaining, header) = result.unwrap();
        assert_eq!(header, expected);
        assert_eq!(remaining, "");
    }

    #[test]
    fn test_parse_header_failure() {
        let input = "Introduction\n";
        let result = parse_header(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_query_success() {
        let input = "- Enhance communication skills.\n";
        let expected = "Enhance communication skills.".to_string();
        let result = parse_query(input);
        assert!(result.is_ok());
        let (remaining, query) = result.unwrap();
        assert_eq!(query, expected);
        assert_eq!(remaining, "");
    }

    #[test]
    fn test_parse_queries_success() {
        let input = "\
- First query.
- Second query.
- Third query.
";
        let expected = vec![
            "First query.".to_string(),
            "Second query.".to_string(),
            "Third query.".to_string(),
        ];
        let result = parse_queries(input);
        assert!(result.is_ok());
        let (remaining, queries) = result.unwrap();
        assert_eq!(queries, expected);
        assert_eq!(remaining, "");
    }

    #[test]
    fn test_parse_single_query_success() {
        let input = "\
- First query.
";
        let expected = vec!["First query.".to_string()];
        let result = parse_queries(input);
        assert!(result.is_ok());
        let (remaining, queries) = result.unwrap();
        assert_eq!(queries, expected);
        assert_eq!(remaining, "");
    }

    #[test]
    fn test_parse_queries_single_query() {
        let input = "- Only one query.\n";
        let expected = vec!["Only one query.".to_string()];
        let result = parse_queries(input);
        assert!(result.is_ok());
        let (remaining, queries) = result.unwrap();
        assert_eq!(queries, expected);
        assert_eq!(remaining, "");
    }

    #[test]
    fn test_parse_note_present() {
        let input = "Note: This is a sample note for testing.\n\n";
        let expected = Some("This is a sample note for testing.".to_string());
        let result = parse_note(input);
        assert!(result.is_ok());
        let (remaining, note) = result.unwrap();
        assert_eq!(note, expected);
        assert_eq!(remaining, "");
    }

    #[test]
    fn test_parse_note_absent() {
        let input = "";
        let expected = None;
        let result = parse_note(input);
        assert!(result.is_ok());
        let (remaining, note) = result.unwrap();
        assert_eq!(note, expected);
        assert_eq!(remaining, "");
    }

    #[test]
    fn test_parse_section_with_note() {
        let input = "\
I. Introduction
- Brief introduction of social media platforms.
- Rise of social media use among the youth.
- The purpose of the essay: to analyze the impact of social media on youth.

Note: Make sure to highlight the prevalence of social media and the importance of discussing its impact on youth.

";
        let expected = Section {
            header: "Introduction".to_string(),
            queries: vec![
                "Brief introduction of social media platforms.".to_string(),
                "Rise of social media use among the youth.".to_string(),
                "The purpose of the essay: to analyze the impact of social media on youth.".to_string(),
            ],
            note: Some("Make sure to highlight the prevalence of social media and the importance of discussing its impact on youth.".to_string()),
        };
        let result = parse_section(input);
        assert!(result.is_ok());
        let (remaining, section) = result.unwrap();
        assert_eq!(section, expected);
        assert_eq!(remaining, "");
    }

    #[test]
    fn test_parse_section_without_note() {
        let input = "\
VI. References
- Cite all sources used to support the arguments in the essay.
";
        let expected = Section {
            header: "References".to_string(),
            queries: vec![
                "Cite all sources used to support the arguments in the essay.".to_string(),
            ],
            note: None,
        };
        let result = parse_section(input);
        assert!(result.is_ok());
        let (remaining, section) = result.unwrap();
        assert_eq!(section, expected);
        assert_eq!(remaining, "");
    }

    #[test]
    fn parse_section_special_case() {
        let input = "\
III. References
- Cite all sources used to support the arguments in the essay.";
        let expected = Section {
            header: "References".to_string(),
            queries: vec![
                "Cite all sources used to support the arguments in the essay.".to_string(),
            ],
            note: None,
        };
        let (input, section) = parse_section(input).unwrap();
        assert_eq!(section, expected);
        assert_eq!(input, "");
    }

    #[test]
    fn parse_sections_success() {
        let input = "\
Outline:

I. Introduction
- Brief introduction of social media platforms.
- Rise of social media use among the youth.
- The purpose of the essay: to analyze the impact of social media on youth.

Note: Make sure to highlight the prevalence of social media and the importance of discussing its impact on youth.

II. Benefits of Social Media on Youth
- Enhancing communication and social skills.
- Gaining knowledge and information.
- Activism and social movements.

Note: Another note.

III. References
- Cite all sources used to support the arguments in the essay.";

        let expected = vec![
            Section {
                header: "Introduction".to_string(),
                queries: vec![
                    "Brief introduction of social media platforms.".to_string(),
                    "Rise of social media use among the youth.".to_string(),
                    "The purpose of the essay: to analyze the impact of social media on youth.".to_string(),
                ],
                note: Some("Make sure to highlight the prevalence of social media and the importance of discussing its impact on youth.".to_string()),
            },
            Section {
                header: "Benefits of Social Media on Youth".to_string(),
                queries: vec![
                    "Enhancing communication and social skills.".to_string(),
                    "Gaining knowledge and information.".to_string(),
                    "Activism and social movements.".to_string(),
                ],
                note: Some("Another note.".to_string()),
            },
            Section {
                header: "References".to_string(),
                queries: vec![
                    "Cite all sources used to support the arguments in the essay.".to_string(),
                ],
                note: None,
            },
        ];

        let result = parse_sections(input);
        assert!(result.is_ok());
        let (remaining, sections) = result.unwrap();
        assert_eq!(sections, expected);
        assert_eq!(remaining, "");
    }

    #[test]
    fn test_parse_section_failure_incorrect_header() {
        let input = "\
Introduction
- First query.
- Second query.

Note: Some note.
";
        let result = parse_section(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_plan_success() {
        let input = "Topic: The Impact of Social Media on Youth

Outline:

I. Introduction
- Brief introduction of social media platforms.
- Rise of social media use among the youth.
- The purpose of the essay: to analyze the impact of social media on youth.

Note: Make sure to highlight the prevalence of social media and the importance of discussing its impact on youth.

II. Benefits of Social Media on Youth
- Enhancing communication and social skills.
- Gaining knowledge and information.
- Activism and social movements.

Note: Another note.

III. References
- Cite all sources used to support the arguments in the essay.
";

        let expected = Plan {
            topic: "The Impact of Social Media on Youth".to_string(),
            sections: vec![
                Section {
                    header: "Introduction".to_string(),
                    queries: vec![
                        "Brief introduction of social media platforms.".to_string(),
                        "Rise of social media use among the youth.".to_string(),
                        "The purpose of the essay: to analyze the impact of social media on youth."
                            .to_string(),
                    ],
                    note: Some("Make sure to highlight the prevalence of social media and the importance of discussing its impact on youth.".to_string()),
                },
                Section {
                    header: "Benefits of Social Media on Youth".to_string(),
                    queries: vec![
                        "Enhancing communication and social skills.".to_string(),
                        "Gaining knowledge and information.".to_string(),
                        "Activism and social movements.".to_string(),
                    ],
                    note: Some("Another note.".to_string()),
                },
                Section {
                    header: "References".to_string(),
                    queries: vec![
                        "Cite all sources used to support the arguments in the essay.".to_string(),
                    ],
                    note: None,
                },
            ],
        };

        let result = parse_plan(input);
        let (input, plan) = result.unwrap();
        println!("Input: `{}`", input);
        assert_eq!(plan, expected);
    }

    #[test]
    fn test_parse_plan_failure_missing_topic() {
        let input = r#"
Outline:

I. Introduction
- Brief introduction of social media platforms.
"#;
        let result = parse_plan(input);
        assert!(result.is_err());
    }
}
