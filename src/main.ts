import { $, chalk, spinner } from 'zx';
import { formatResult, getCommonLines, printResult } from './utils.js';

$.verbose = false;

async function main() {
  const [allUpdates, explicitPackages] = await Promise.all([
    $`checkupdates & yay -Qua & wait`.pipe($`sort`).quiet(),
    $`yay -Qe`
      .pipe($`awk -F' ' '{print $1}'`)
      .pipe($`sort`)
      .quiet(),
  ]);
  const file1Lines = allUpdates.toString().split('\n');
  const file2Lines = explicitPackages.toString().split('\n');
  const commonLines = getCommonLines(file1Lines, file2Lines);
  const result = formatResult(commonLines);

  printResult(result);

  console.log(`${chalk.bold('Total updates:')} ${commonLines.length}`);
}

await spinner('Loading', () => main());
