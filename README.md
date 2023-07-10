# JSON Response

## Instructions

There is a file in the root of the assessment with a list of [popular languages](https://survey.stackoverflow.co/2023/?utm_source=so-owned&utm_medium=blog&utm_campaign=dev-survey-results-2023&utm_content=survey-results#technology-most-popular-technologies). Create a route at `/` and return the list in a JSON object.

You will only need to update the route handler [favorite_languages.rs](src/router/favorite_languages.rs) to complete this assessment.

## Skills

The following skills are being assessed

- Reading from a file
- Returning JSON from a route handler
- Converting a string to a Rust object

## Grading Rubric

### Check script passing

- Server still returns "Hello world"
- Code is formatted
- Code passes Clippy lint

### Manual check

- favorite languages are returned as a json array of objects
