namespace AdventOfCode2022.Days
{
    static class Day10
    {
        public static void Solve()
        {
            Console.WriteLine("Day 10, part 2:");
            Console.WriteLine("Day 10, part 1: " + Solve(FullInput));
            Console.WriteLine();
        }

        private static int Solve(string input)
        {
            var X = 1;
            var tick = 0;
            var ticksToCheck = new Queue<int>(new[] { 20, 60, 100, 140, 180, 220 });

            var result = 0;
            foreach (var command in input.Split(Environment.NewLine, StringSplitOptions.RemoveEmptyEntries).Select(x => x.Trim()))
            {
                if (command == "noop")
                {
                    DrawPixel();
                    tick++;
                    if (ticksToCheck.Any() && ticksToCheck.Peek() == tick)
                        result += X * ticksToCheck.Dequeue(); ;
                }
                else // addx V - very confusing description...
                {
                    if (ticksToCheck.Any() && (tick == ticksToCheck.Peek() - 1 || tick == ticksToCheck.Peek() - 2))
                        result += X * ticksToCheck.Dequeue();
                    DrawPixel();
                    tick++;
                    DrawPixel();
                    tick++;
                    X += int.Parse(command.Split(' ')[1]);
                }
            }

            return result;

            void DrawPixel()
            {
                if (new[] { X - 1, X, X + 1 }.Contains(tick % 40))
                {
                    Console.Write('#');
                }
                else
                {
                    Console.Write('.');
                }

                if ((tick + 1) % 40 == 0)
                    Console.Write(Environment.NewLine);
            }
        }

        #region Input

        private const string ShortInput = @"
            addx 15
            addx -11
            addx 6
            addx -3
            addx 5
            addx -1
            addx -8
            addx 13
            addx 4
            noop
            addx -1
            addx 5
            addx -1
            addx 5
            addx -1
            addx 5
            addx -1
            addx 5
            addx -1
            addx -35
            addx 1
            addx 24
            addx -19
            addx 1
            addx 16
            addx -11
            noop
            noop
            addx 21
            addx -15
            noop
            noop
            addx -3
            addx 9
            addx 1
            addx -3
            addx 8
            addx 1
            addx 5
            noop
            noop
            noop
            noop
            noop
            addx -36
            noop
            addx 1
            addx 7
            noop
            noop
            noop
            addx 2
            addx 6
            noop
            noop
            noop
            noop
            noop
            addx 1
            noop
            noop
            addx 7
            addx 1
            noop
            addx -13
            addx 13
            addx 7
            noop
            addx 1
            addx -33
            noop
            noop
            noop
            addx 2
            noop
            noop
            noop
            addx 8
            noop
            addx -1
            addx 2
            addx 1
            noop
            addx 17
            addx -9
            addx 1
            addx 1
            addx -3
            addx 11
            noop
            noop
            addx 1
            noop
            addx 1
            noop
            noop
            addx -13
            addx -19
            addx 1
            addx 3
            addx 26
            addx -30
            addx 12
            addx -1
            addx 3
            addx 1
            noop
            noop
            noop
            addx -9
            addx 18
            addx 1
            addx 2
            noop
            noop
            addx 9
            noop
            noop
            noop
            addx -1
            addx 2
            addx -37
            addx 1
            addx 3
            noop
            addx 15
            addx -21
            addx 22
            addx -6
            addx 1
            noop
            addx 2
            addx 1
            noop
            addx -10
            noop
            noop
            addx 20
            addx 1
            addx 2
            addx 2
            addx -6
            addx -11
            noop
            noop
            noop";

        private const string FullInput = @"
            noop
            noop
            noop
            addx 4
            addx 3
            addx 3
            addx 3
            noop
            addx 2
            addx 1
            addx -7
            addx 10
            addx 1
            addx 5
            addx -3
            addx -7
            addx 13
            addx 5
            addx 2
            addx 1
            addx -30
            addx -8
            noop
            addx 3
            addx 2
            addx 7
            noop
            addx -2
            addx 5
            addx 2
            addx -7
            addx 8
            addx 2
            addx 5
            addx 2
            addx -12
            noop
            addx 17
            addx 3
            addx -2
            addx 2
            noop
            addx 3
            addx -38
            noop
            addx 3
            addx 4
            noop
            addx 5
            noop
            noop
            noop
            addx 1
            addx 2
            addx 5
            addx 2
            addx -3
            addx 4
            addx 2
            noop
            noop
            addx 7
            addx -30
            addx 31
            addx 4
            noop
            addx -24
            addx -12
            addx 1
            addx 5
            addx 5
            noop
            noop
            noop
            addx -12
            addx 13
            addx 4
            noop
            addx 23
            addx -19
            addx 1
            addx 5
            addx 12
            addx -28
            addx 19
            noop
            addx 3
            addx 2
            addx 5
            addx -40
            addx 4
            addx 32
            addx -31
            noop
            addx 13
            addx -8
            addx 5
            addx 2
            addx 5
            noop
            noop
            noop
            addx 2
            addx -7
            addx 8
            addx -7
            addx 14
            addx 3
            addx -2
            addx 2
            addx 5
            addx -40
            noop
            noop
            addx 3
            addx 4
            addx 1
            noop
            addx 2
            addx 5
            addx 2
            addx 21
            noop
            addx -16
            addx 3
            noop
            addx 2
            noop
            addx 1
            noop
            noop
            addx 4
            addx 5
            noop
            noop
            noop
            noop
            noop
            noop
            noop";

        #endregion
    }
}
