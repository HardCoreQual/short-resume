# Text Summarization Script with OpenAI API

This script is designed to create a simple summary of a given text using the OpenAI API. It can be useful for creating concise versions of long articles or documents.

**Disclaimer**: This script is not intended for real-world use. It is a result of personal experimentation and learning.


## Prerequisites

- Rust Programming Language: Make sure you have Rust installed on your machine. If not, you can download it from the official [Rust website](https://www.rust-lang.org/).
- OpenAI API key: You will need an API key from OpenAI to access the API. Sign up on the [OpenAI platform](https://www.openai.com/) to get your key.

## How to Run

To run the script, use the following command:

```bash
OPENAI_API_KEY=YOUR_API_KEY cargo run URL
```

Replace `YOUR_API_KEY` with your actual OpenAI API key and `URL` with the URL of the text you want to summarize.

For example:

```bash
OPENAI_API_KEY=SECRET cargo run https://help.openai.com/en/articles/7127966-what-is-the-difference-between-the-gpt-4-models
```

This will run the script and print the summarized version of the text found at the provided URL.
