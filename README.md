# Tidefi Primitives

Low-level types used throughout the Tidefi ecosystem.

## Generate JSON Schema

```
cargo install --git https://github.com/tidelabs/primitives --force
tidefi-primitives json -o ./dist
```

## Parse latest JSON Schema

By example to view all supported assets:

```
curl -Ls https://github.com/tidelabs/primitives/releases/latest/download/assets.json | jq '.[] | [.id,.name,.abbr,.exponent]'
```

View available networks:

```
curl -Ls https://github.com/tidelabs/primitives/releases/latest/download/networks.json | jq '.[].name'
```

#### License

<sup>
The entire code within this repository is licensed under the <a href="LICENSE">GPLv3</a>.
</sup>
