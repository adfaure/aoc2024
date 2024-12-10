const fs = require("fs");

const input = fs.readFileSync("./input", "utf-8");
const line = input.split("\n");

let result = 0;
let word = "XMAS";
const directions = [
  [-1, 0],
  [1, 0],
  [0, -1],
  [0, 1],
  [-1, -1],
  [-1, 1],
  [1, -1],
  [1, 1],
];

const isValid = (x, y) =>
  y >= 0 && y < line.length && x >= 0 && x < line[0].length;

const findWord = (x, y, word, index) => {
  console.log("\t", x, y);

  if (line[y][x] !== word[index]) { 
					return 0; 
	}

  if (index === word.length) {
					return 1;
	}

  let matches = 0;
  for (let [dx, dy] of directions) {
    if (!isValid(x + dx, y + dy)) continue;
    matches += findWord(x + dx, y + dy, word, index + 1)
  }
  return matches;
};

for (let j = 0; j < line.length; j++) {
  for (let i = 0; i < line[j].length; i++) {
    if (line[j][i] === "X") {
      console.log(i, j);
      let res = findWord(i, j, word, 0);
      if (res > 0) {
        result += res;
      }
    }
  }
}

console.log(result);
