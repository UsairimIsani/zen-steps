# Zen Steps Backend

Zen steps used OpenAI chat completion and text-to-speech recognition APIs

To use te backend a valid OpenAI token is required

Set env variable for the backend

```bash
source OPENAI_API_BASE="https://api.openai.com/v1"
source OPENAI_API_KEY="<OPEN_API_TOKEN>"
source OPENAI_API_TEXT_TO_SPEECH_MODEL_ENDPOINT="https://api.openai.com/v1/audio/speech"
source PROJECT_NAME="Zen Steps"
```

```bash
cargo run --release
```
