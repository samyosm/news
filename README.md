# News TUI
[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg?style=for-the-badge)](https://www.gnu.org/licenses/gpl-3.0)
[![Twitter Follow](https://img.shields.io/twitter/follow/samy_osmium?style=for-the-badge)](https://twitter.com/intent/follow?screen_name=samy_osmium)
[![GitHub issues](https://img.shields.io/github/issues/samyosm/habitify-cli?style=for-the-badge)](https://github.com/samyosm/news/issues)

News is a text user interface that allows you to consume daily news in multiple categories like technology, science, health, and so on.

![Screenshot 2023-07-15 03-21-17](https://github.com/samyosm/news/assets/99157490/221aa1ed-b892-4e87-8995-189ebb899f7b)

## Insallation
### Using Cargo
```sh
cargo install news
```

### Manual Build
```sh
git clone https://github.com/samyosm/news.git
cd news
cargo build
```

## Usage

```sh
news
```

### Home Page
| key          | description                    |
| ------------ | ------------------------------ |
| j, down      | Go down                        |
| k, up        | Go up                          |
| l, tab       | Switch category to the right   |
| h, shift+tab | Switch category to the left    |
| enter        | view the content of an article |

### Article view
| key | description          |
| --- | -------------------- |
| esc | go back to home page |

## License
[GPL-3.0](./LICENSE)
