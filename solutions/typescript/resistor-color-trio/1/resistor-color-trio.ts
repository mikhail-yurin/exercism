type Color = 'black' | 'brown' | 'red' | 'orange' | 'yellow' | 'green' | 'blue' | 'violet' | 'grey' | 'white';

export const getColorCode = (color: Color) => {
  switch (color) {
      case 'black':
        return 0;
      case 'brown':
        return 1;
      case 'red':
        return 2;
      case 'orange':
        return 3;
      case 'yellow':
        return 4;
      case 'green':
        return 5;
      case 'blue':
        return 6;
      case 'violet':
        return 7;
      case 'grey':
        return 8;
      case 'white':
        return 9;
    }
}

export function decodedResistorValue(colors: Color[]) {
  const zeros = getColorCode(colors[2]);
  const combination: string = `${getColorCode(colors[0])}${getColorCode(colors[1])}${'0'.repeat(zeros)}`;

  const num = parseInt(combination, 10);

  const giga = 1000000000;
  const mega = 1000000;
  const kilo = 1000;

  if (!num) return `${num} ohms`;
  if (!(num % giga)) return `${num / giga} gigaohms`;
  if (!(num % mega)) return `${num / mega} megaohms`;
  if (!(num % kilo)) return `${num / kilo} kiloohms`;
  return `${num} ohms`;
}
