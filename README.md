# pg_tiktoken

Postgres extension that does input tokenization using OpenAI's tiktoken.

## Usage

```sql
db=> create extension pg_tiktoken;
CREATE EXTENSION
db=> select tiktoken_count('p50k_edit', 'A long time ago in a galaxy far, far away');
 tiktoken_count 
----------------
             11
(1 row)

db=> select tiktoken_encode('cl100k_base', 'A long time ago in a galaxy far, far away');
                  tiktoken_encode                   
----------------------------------------------------
 {32,1317,892,4227,304,264,34261,3117,11,3117,3201}
(1 row)
```

## Testing
```bash
# build and install pg_tiktoken
git clone https://github.com/neondatabase/pg_tiktoken
cd pg_tiktoken
cargo pgrx install
cargo pgrx run pg17
```

## Supported models

| Encoding name           | OpenAI models                                       |
|-------------------------|-----------------------------------------------------|
| `cl100k_base`           | ChatGPT models, `text-embedding-ada-002`            |
| `p50k_base`             | Code models, `text-davinci-002`, `text-davinci-003` |
| `p50k_edit`             | Use for edit models like `text-davinci-edit-001`, `code-davinci-edit-001` |
| `r50k_base` (or `gpt2`) | GPT-3 models like `davinci`                         |

`tiktoken_count` and `tiktoken_encode` functions accept both encoding name and OpenAI model name as a first argument.


## Installation

Assuming that rust toolchain is already istalled:

```sh
# build and install pg_tiktoken
git clone https://github.com/neondatabase/pg_tiktoken
cd pg_tiktoken
cargo pgrx install
```

## Kudos

- https://github.com/zurawiki/tiktoken-rs
- https://github.com/openai/tiktoken
