#!/usr/bin/env -S deno run --allow-read --allow-write

import Ask from "https://deno.land/x/ask/mod.ts";
import * as toml from "https://deno.land/std@0.84.0/encoding/toml.ts";

const ask = new Ask();
const config: any = await Deno.readTextFile("Cargo.toml").then((txt) =>
  toml.parse(txt)
).catch((e) => console.error(e));

const { pkg, author } = await ask.prompt([
  {
    name: "pkg",
    type: "input",
    message: `Package name? (${config.package.name})`,
  },
  {
    name: "author",
    type: "input",
    message: `Author? (${config.package.authors[0]})`,
  },
]);

config.package.name = pkg ?? config.package.name;
config.package.authors = author ? [author] : config.package.authors;

await Deno.writeTextFile("Cargo.toml", toml.stringify(config));
console.log(
  "Successfully updated Cargo.toml! Run your Nannou app with `cargo run --release`",
);
