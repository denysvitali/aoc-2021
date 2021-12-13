import { partOne, partTwo } from './solution.ts';

type SampleOrInput = "sample" | "input";

async function parseInput(fileName: string): Promise<string> {
    return await Deno.readTextFile(fileName);
}

function getFilename(solution: SampleOrInput): string {
  switch (solution) {
    case "sample":
      return "input/sample.txt";
    case "input":
      return "input/input.txt";
  }
}

const args = Deno.args;

if (args.length != 1) {
  console.error(`Usage: sample|input`);
  Deno.exit(1);
}

if (args[0] == 'input' || args[0] == 'sample') {
    const sampleInput: SampleOrInput = args[0];
    const input = parseInput(getFilename(sampleInput));

    console.log(`Part One: ` + partOne(await input));
    console.log(`Part Two: ` + partTwo(await input));
} else {
    console.error(`Invalid argument provided. Please provide either "sample" or "input".`)
    Deno.exit(1);
}