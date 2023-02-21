using System.Numerics;

namespace AdventOfCode2022.Days
{
    // I hate this challenge simply for the fact that I had to manually write the input
    static class Day11
    {
        /* `Chinese Remainder Theorem` (aka bullshit maths stuff)
         * 
         * We take our worry level `w` and check the remainder of it modulo some number
         * (short input uses 13, 17, 19, 23). Because `w` can grow too large, it would
         * be useful to find a way to make it smaller (part 1 divides it by 3).
         * 
         * The theorem states that for divisors that are coprime (like ours), we can
         * multiply the divisors and solve what the value of `n modulo product` is,
         * without actually knowing the value of `n` itself.
         * 
         * In our case, however, we know already know that `n` is the worry level `w`,
         * so we know that `w` and `w modulo product` will behave the same when tested
         * with each monkey's divisor. So we'll use this to relieve the stress:
         * 
         *      `w = w modulo divisorProduct`
         */
        public static void Solve()
        {
            Console.WriteLine("Day 11, part 1: " + Solve(FullInput(), 20, x => x / 3));
            Console.WriteLine("Day 11, part 2: " + Solve(FullInput(), 10000, x => x % FullInputDivisorProduct)); // Some mathematical bullshit, see `Chinese Remainder Theorem`
            Console.WriteLine();
        }

        private static BigInteger Solve(Monkey[] monkeys, int rounds, Func<BigInteger, BigInteger> relieveStress)
        {
            var monkeyInspectionCount = new BigInteger[monkeys.Length];
            Array.Fill(monkeyInspectionCount, 0);
            for (int round = 0; round < rounds; round++)
            {
                foreach (var monkey in monkeys)
                {
                    while (monkey.Items.Any())
                    {
                        var item = monkey.Items.Dequeue();
                        monkeyInspectionCount[monkey.Id]++;

                        item = relieveStress(monkey.Operation(item));

                        if (monkey.Test(item))
                        {
                            monkeys[monkey.TargetTrue].Items.Enqueue(item);
                        }
                        else
                        {
                            monkeys[monkey.TargetFalse].Items.Enqueue(item);
                        }
                    }
                }
            }

            Array.Sort(monkeyInspectionCount, (a, b) => a > b ? -1 : 1); // Sort descendingly
            return monkeyInspectionCount[0] * monkeyInspectionCount[1];
        }

        #region Input

        private static Monkey[] ShortInput() => new[]
        {
            new Monkey
            {
                Id = 0,
                Items = new Queue<BigInteger>(new BigInteger[] { 79, 98 }),
                Operation = old => old * 19,
                Test = value => value % 23 == 0,
                TargetTrue = 2,
                TargetFalse = 3
            },
            new Monkey
            {
                Id = 1,
                Items = new Queue<BigInteger>(new BigInteger[] { 54ul, 65ul, 75ul, 74ul }),
                Operation = old => old + 6,
                Test = value => value % 19 == 0,
                TargetTrue = 2,
                TargetFalse = 0
            },
            new Monkey
            {
                Id = 2,
                Items = new Queue<BigInteger>(new BigInteger[] { 79ul, 60ul, 97ul }),
                Operation = old => old * old,
                Test = value => value % 13 == 0,
                TargetTrue = 1,
                TargetFalse = 3
            },
            new Monkey
            {
                Id = 3,
                Items = new Queue<BigInteger>(new BigInteger[] { 74ul }),
                Operation = old => old + 3,
                Test = value => value % 17 == 0,
                TargetTrue = 0,
                TargetFalse = 1
            }
        };
        private static readonly int ShortInputDivisorProduct = 13 * 17 * 19 * 23;

        private static Monkey[] FullInput() => new[]
        {
            new Monkey
            {
                Id = 0,
                Items = new Queue<BigInteger>(new BigInteger[] { 74ul, 73ul, 57ul, 77ul, 74ul }),
                Operation = old => old * 11,
                Test = value => value % 19 == 0,
                TargetTrue = 6,
                TargetFalse = 7
            },
            new Monkey
            {
                Id = 1,
                Items = new Queue<BigInteger>(new BigInteger[] { 99ul, 77ul, 79ul }),
                Operation = old => old + 8,
                Test = value => value % 2 == 0,
                TargetTrue = 6,
                TargetFalse = 0
            },
            new Monkey
            {
                Id = 2,
                Items = new Queue<BigInteger>(new BigInteger[] { 64ul, 67ul, 50ul, 96ul, 89ul, 82ul, 82ul }),
                Operation = old => old + 1,
                Test = value => value % 3 == 0,
                TargetTrue = 5,
                TargetFalse = 3
            },
            new Monkey
            {
                Id = 3,
                Items = new Queue<BigInteger>(new BigInteger[] { 88ul }),
                Operation = old => old * 7,
                Test = value => value % 17 == 0,
                TargetTrue = 5,
                TargetFalse = 4
            },
            new Monkey
            {
                Id = 4,
                Items = new Queue<BigInteger>(new BigInteger[] { 80ul, 66ul, 98ul, 83ul, 70ul, 63ul, 57ul, 66ul }),
                Operation = old => old + 4,
                Test = value => value % 13 == 0,
                TargetTrue = 0,
                TargetFalse = 1
            },
            new Monkey
            {
                Id = 5,
                Items = new Queue<BigInteger>(new BigInteger[] { 81ul, 93ul, 90ul, 61ul, 62ul, 64ul }),
                Operation = old => old + 7,
                Test = value => value % 7 == 0,
                TargetTrue = 1,
                TargetFalse = 4
            },
            new Monkey
            {
                Id = 6,
                Items = new Queue<BigInteger>(new BigInteger[] { 69ul, 97ul, 88ul, 93ul }),
                Operation = old => old * old,
                Test = value => value % 5 == 0,
                TargetTrue = 7,
                TargetFalse = 2
            },
            new Monkey
            {
                Id = 7,
                Items = new Queue<BigInteger>(new BigInteger[] { 59ul, 80ul }),
                Operation = old => old + 6,
                Test = value => value % 11 == 0,
                TargetTrue = 2,
                TargetFalse = 3
            }
        };
        private static readonly int FullInputDivisorProduct = 19 * 2 * 3 * 17 * 13 * 7 * 5 * 11;

        #endregion
    }

    class Monkey
    {
        public int Id { get; init; }
        public Queue<BigInteger> Items { get; init; }
        public Func<BigInteger, BigInteger> Operation { get; init; }
        public Func<BigInteger, bool> Test { get; init; }
        public int TargetTrue { get; init; }
        public int TargetFalse { get; init; }
    }
}
