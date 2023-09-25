import { chalk } from 'zx';

export function getVersionParts(version: string) {
  return version.split('.').map((part) => parseInt(part));
}

export function compareVersions(oldVersion: string, newVersion: string) {
  const oldParts = getVersionParts(oldVersion);
  const newParts = getVersionParts(newVersion);

  if (oldParts[0] !== newParts[0]) {
    return 'major';
  } else if (oldParts[1] !== newParts[1]) {
    return 'minor';
  } else if (oldParts[2] !== newParts[2]) {
    return 'patch';
  } else {
    return 'build';
  }
}

export function formatVersionColor(
  version: string,
  updateType: 'major' | 'minor' | 'patch' | 'build',
) {
  const verParts = version.split('.');

  switch (updateType) {
    case 'major':
      return chalk.bold.red(verParts.join('.'));
    case 'minor':
      return `${verParts[0]}.${chalk.bold.yellow(verParts.slice(1).join('.'))}`;
    case 'patch':
      return `${verParts[0]}.${verParts[1]}.${chalk.bold.green(
        verParts.slice(2).join('.'),
      )}`;
    case 'build':
      return verParts.join('.');
  }
}
export function getCommonLines(file1Lines: string[], file2Lines: string[]) {
  const commonLines: string[] = [];
  let i = 0;
  let j = 0;

  while (i < file1Lines.length && j < file2Lines.length) {
    if (file1Lines[i].includes(file2Lines[j])) {
      commonLines.push(file1Lines[i]);
      i++;
      j++;
    } else if (file1Lines[i] < file2Lines[j]) {
      i++;
    } else {
      j++;
    }
  }

  return commonLines;
}

type UpdateTypeMap = {
  major: string[];
  minor: string[];
  patch: string[];
  build: string[];
};

export function formatResult(commonLines: string[]) {
  const updateTypeMap: UpdateTypeMap = {
    major: [],
    minor: [],
    patch: [],
    build: [],
  };

  for (const line of commonLines) {
    const l = line.split(' ');
    const packageName = chalk.bold(l[0]);
    const oldVersion = l[1];
    const newVersion = l[3];
    const updateType = compareVersions(oldVersion, newVersion);

    updateTypeMap[updateType].push(
      `${packageName}: ${formatVersionColor(
        oldVersion,
        updateType,
      )}  ${formatVersionColor(newVersion, updateType)}`,
    );
  }

  return updateTypeMap;
}

export function printResult(updateTypeMap: UpdateTypeMap) {
  for (const [key, value] of Object.entries(updateTypeMap)) {
    if (value.length > 0) {
      console.log(
        chalk.bold.underline(key.toUpperCase()),
        chalk.greenBright(`(${value.length})`),
        '     ',
      );
      value.forEach((it) => console.log(it));
      console.log();
    }
  }
}
