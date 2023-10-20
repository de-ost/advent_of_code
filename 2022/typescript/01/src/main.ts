// https://adventofcode.com/2022/day/1

import * as readline from "readline";
import * as fs from "fs";

const FILE_PATH = "./resources/input.txt";

async function main() {
  let calories = await readFile(FILE_PATH);
  calories.sort((a, b) => b - a);

  // Part 1
  const max = calories[0];
  console.log(`max = ${max}`);

  // Part 2
  const sumOfThree = calories.slice(0, 3).reduce((acc, curr) => acc + curr);
  console.log(`summ of 3 = ${sumOfThree}`);
}

/**
 * Reads file line by line and returns an array of calories each elf is carrying
 * 
 * @param path path to file with list of calories
 * @returns array of calories
 */
async function readFile(path: string): Promise<number[]> {
  const rl = readline.createInterface({
    input: fs.createReadStream(path),
    crlfDelay: Infinity,
  });

  const calories: number[] = [];
  let currentCalories = 0;

  for await (const line of rl) {
    if (line === "") {
      calories.push(currentCalories);
      currentCalories = 0;
    } else {
      currentCalories += parseInt(line);
    }
  }

  calories.push(currentCalories);
  return calories;
}

main();
