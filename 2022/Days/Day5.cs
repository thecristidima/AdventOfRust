﻿using System.Text.RegularExpressions;

namespace AdventOfCode2022.Days
{
    // Input was manually changed because it was small and the format for the stacks is kinda dumb
    static class Day5
    {
        public static void Solve()
        {
            Console.WriteLine("Day 5, part 1: " + Solve(FullInput));
            Console.WriteLine("Day 5, part 2: " + Solve(FullInput, reverseStack: true));
        }

        private static string Solve(string input, bool reverseStack = false)
        {
            var lines = input.Split(Environment.NewLine, StringSplitOptions.RemoveEmptyEntries).Select(x => x.Trim());
            var stacks = lines.Where(x => !x.StartsWith("move")).Select(ParseStackLine).ToArray();
            var moves = lines.Where(x => x.StartsWith("move"));

            foreach (var move in moves)
            {
                (var amount, var source, var destination) = ParseMoveCommand(move);
                if (reverseStack)
                {
                    var temp = new Stack<string>();
                    for (int i = 0; i < amount; i++)
                        temp.Push(stacks[source - 1].Pop());
                    for (int i = 0; i < amount; i++)
                        stacks[destination - 1].Push(temp.Pop());
                }
                else
                {
                    for (int i = 0; i < amount; i++)
                    {
                        stacks[destination - 1].Push(stacks[source - 1].Pop());
                    }
                }
            }

            var result = string.Empty;
            foreach (var stack in stacks)
                result += stack.Peek();

            return result;
        }

        private static Stack<string> ParseStackLine(string stackLine)
            => new(stackLine.Split(" ", StringSplitOptions.RemoveEmptyEntries));

        private static (int, int, int) ParseMoveCommand(string command)
        {
            var match = Regex.Match(command, "move (\\d+) from (\\d+) to (\\d+)");
            return (int.Parse(match.Groups[1].Value), int.Parse(match.Groups[2].Value), int.Parse(match.Groups[3].Value));
        }

        #region Input

        private const string SmallInput = @"
            Z N
            M C D
            P

            move 1 from 2 to 1
            move 3 from 1 to 3
            move 2 from 2 to 1
            move 1 from 1 to 2";

        private const string FullInput = @"
            Q F M R L W C V
            D Q L
            P S R G W C N B
            L C D H B Q G
            V G L F Z S
            D G N P
            D Z P V F C W
            C P D M S
            Z N W T V M P C

            move 1 from 9 to 2
            move 4 from 6 to 1
            move 4 from 2 to 6
            move 5 from 8 to 7
            move 4 from 9 to 2
            move 1 from 5 to 8
            move 1 from 3 to 1
            move 2 from 3 to 1
            move 1 from 4 to 2
            move 11 from 7 to 2
            move 5 from 5 to 1
            move 1 from 6 to 8
            move 1 from 7 to 6
            move 3 from 6 to 7
            move 1 from 3 to 2
            move 1 from 6 to 8
            move 11 from 2 to 1
            move 1 from 9 to 8
            move 1 from 3 to 7
            move 4 from 7 to 9
            move 3 from 3 to 7
            move 4 from 8 to 2
            move 3 from 7 to 6
            move 2 from 6 to 3
            move 5 from 4 to 1
            move 1 from 6 to 5
            move 26 from 1 to 7
            move 1 from 4 to 6
            move 22 from 7 to 5
            move 4 from 9 to 1
            move 3 from 7 to 3
            move 1 from 6 to 3
            move 6 from 1 to 7
            move 2 from 7 to 5
            move 8 from 1 to 9
            move 4 from 3 to 4
            move 10 from 2 to 7
            move 6 from 7 to 4
            move 2 from 9 to 5
            move 1 from 5 to 1
            move 8 from 4 to 1
            move 2 from 5 to 9
            move 1 from 3 to 6
            move 1 from 9 to 1
            move 1 from 3 to 6
            move 2 from 5 to 2
            move 1 from 4 to 2
            move 1 from 2 to 3
            move 7 from 1 to 4
            move 9 from 7 to 4
            move 1 from 3 to 4
            move 2 from 2 to 4
            move 5 from 9 to 6
            move 1 from 4 to 5
            move 2 from 9 to 3
            move 1 from 1 to 6
            move 2 from 6 to 1
            move 2 from 6 to 5
            move 2 from 9 to 7
            move 1 from 3 to 9
            move 1 from 9 to 5
            move 2 from 7 to 3
            move 1 from 1 to 7
            move 7 from 4 to 5
            move 2 from 1 to 2
            move 3 from 3 to 8
            move 3 from 8 to 9
            move 31 from 5 to 8
            move 1 from 7 to 1
            move 1 from 2 to 1
            move 1 from 1 to 5
            move 1 from 5 to 6
            move 2 from 5 to 7
            move 10 from 4 to 9
            move 5 from 6 to 2
            move 3 from 2 to 6
            move 2 from 7 to 8
            move 1 from 6 to 3
            move 1 from 4 to 1
            move 1 from 3 to 6
            move 1 from 4 to 2
            move 2 from 1 to 2
            move 1 from 8 to 7
            move 10 from 8 to 2
            move 13 from 2 to 9
            move 1 from 1 to 5
            move 18 from 8 to 2
            move 21 from 9 to 6
            move 1 from 7 to 8
            move 2 from 9 to 7
            move 1 from 2 to 3
            move 1 from 7 to 8
            move 9 from 2 to 4
            move 1 from 7 to 8
            move 3 from 9 to 1
            move 1 from 8 to 1
            move 6 from 2 to 3
            move 5 from 4 to 7
            move 1 from 5 to 8
            move 2 from 4 to 3
            move 5 from 7 to 3
            move 2 from 2 to 7
            move 15 from 6 to 1
            move 12 from 1 to 2
            move 6 from 2 to 9
            move 4 from 9 to 5
            move 4 from 5 to 6
            move 14 from 3 to 9
            move 1 from 6 to 7
            move 1 from 7 to 2
            move 1 from 7 to 8
            move 9 from 2 to 6
            move 1 from 1 to 6
            move 2 from 9 to 8
            move 4 from 9 to 7
            move 1 from 1 to 5
            move 8 from 8 to 3
            move 1 from 5 to 4
            move 2 from 1 to 2
            move 3 from 1 to 4
            move 9 from 6 to 2
            move 1 from 7 to 4
            move 1 from 8 to 2
            move 1 from 6 to 4
            move 4 from 7 to 8
            move 12 from 6 to 8
            move 3 from 2 to 1
            move 6 from 8 to 7
            move 5 from 3 to 6
            move 3 from 3 to 6
            move 3 from 1 to 3
            move 8 from 2 to 9
            move 2 from 4 to 5
            move 2 from 7 to 2
            move 10 from 8 to 5
            move 3 from 3 to 2
            move 10 from 5 to 3
            move 1 from 4 to 3
            move 1 from 2 to 1
            move 1 from 1 to 7
            move 14 from 9 to 6
            move 5 from 2 to 4
            move 15 from 6 to 5
            move 3 from 9 to 3
            move 1 from 8 to 6
            move 1 from 3 to 8
            move 7 from 3 to 8
            move 16 from 5 to 1
            move 2 from 7 to 1
            move 1 from 5 to 9
            move 2 from 9 to 3
            move 15 from 1 to 5
            move 3 from 8 to 2
            move 3 from 3 to 1
            move 3 from 7 to 3
            move 8 from 4 to 6
            move 5 from 1 to 6
            move 9 from 5 to 7
            move 2 from 8 to 3
            move 2 from 2 to 7
            move 1 from 1 to 4
            move 2 from 5 to 8
            move 4 from 3 to 1
            move 4 from 8 to 1
            move 1 from 8 to 6
            move 9 from 7 to 6
            move 2 from 7 to 5
            move 3 from 1 to 8
            move 1 from 4 to 8
            move 1 from 2 to 4
            move 12 from 6 to 2
            move 3 from 8 to 6
            move 1 from 4 to 7
            move 2 from 6 to 8
            move 5 from 5 to 9
            move 13 from 2 to 9
            move 2 from 4 to 7
            move 13 from 9 to 5
            move 2 from 6 to 5
            move 1 from 3 to 9
            move 6 from 9 to 4
            move 5 from 1 to 3
            move 1 from 7 to 9
            move 15 from 5 to 8
            move 2 from 4 to 7
            move 2 from 4 to 6
            move 1 from 4 to 6
            move 1 from 5 to 7
            move 18 from 6 to 2
            move 2 from 7 to 3
            move 3 from 6 to 7
            move 3 from 2 to 8
            move 5 from 7 to 3
            move 1 from 9 to 6
            move 2 from 3 to 8
            move 11 from 3 to 2
            move 2 from 2 to 9
            move 1 from 6 to 2
            move 1 from 7 to 5
            move 1 from 5 to 9
            move 9 from 8 to 4
            move 1 from 4 to 6
            move 2 from 3 to 1
            move 2 from 1 to 5
            move 12 from 8 to 3
            move 1 from 8 to 2
            move 14 from 3 to 4
            move 1 from 6 to 4
            move 1 from 5 to 4
            move 20 from 2 to 7
            move 2 from 9 to 5
            move 1 from 5 to 3
            move 1 from 9 to 2
            move 1 from 2 to 8
            move 2 from 2 to 3
            move 5 from 4 to 5
            move 6 from 5 to 7
            move 2 from 8 to 2
            move 3 from 3 to 9
            move 5 from 4 to 5
            move 2 from 9 to 7
            move 2 from 2 to 3
            move 1 from 9 to 3
            move 22 from 7 to 3
            move 4 from 7 to 4
            move 24 from 3 to 6
            move 4 from 2 to 6
            move 18 from 6 to 9
            move 15 from 4 to 6
            move 8 from 6 to 3
            move 6 from 6 to 1
            move 7 from 9 to 6
            move 2 from 7 to 4
            move 8 from 3 to 9
            move 14 from 6 to 3
            move 2 from 3 to 9
            move 1 from 9 to 6
            move 13 from 9 to 1
            move 3 from 4 to 5
            move 1 from 9 to 6
            move 5 from 1 to 8
            move 3 from 3 to 9
            move 2 from 1 to 5
            move 8 from 5 to 8
            move 10 from 3 to 5
            move 3 from 4 to 6
            move 6 from 1 to 9
            move 4 from 5 to 3
            move 5 from 8 to 2
            move 6 from 6 to 3
            move 7 from 3 to 6
            move 1 from 3 to 4
            move 5 from 8 to 7
            move 5 from 2 to 6
            move 2 from 7 to 3
            move 3 from 7 to 3
            move 1 from 4 to 9
            move 9 from 6 to 9
            move 2 from 6 to 2
            move 1 from 8 to 2
            move 2 from 8 to 7
            move 5 from 1 to 5
            move 1 from 1 to 4
            move 13 from 5 to 7
            move 5 from 3 to 7
            move 1 from 5 to 6
            move 1 from 4 to 6
            move 3 from 2 to 8
            move 1 from 3 to 5
            move 1 from 3 to 8
            move 14 from 7 to 4
            move 1 from 5 to 6
            move 7 from 6 to 9
            move 6 from 7 to 9
            move 2 from 8 to 9
            move 2 from 8 to 1
            move 31 from 9 to 1
            move 13 from 4 to 2
            move 1 from 4 to 3
            move 10 from 2 to 7
            move 1 from 3 to 4
            move 1 from 2 to 7
            move 3 from 7 to 8
            move 1 from 4 to 1
            move 3 from 8 to 5
            move 32 from 1 to 5
            move 3 from 9 to 7
            move 4 from 9 to 6
            move 2 from 2 to 7
            move 2 from 1 to 7
            move 1 from 6 to 1
            move 1 from 9 to 4
            move 3 from 6 to 4
            move 1 from 1 to 8
            move 15 from 5 to 1
            move 1 from 8 to 4
            move 9 from 5 to 7
            move 1 from 9 to 8
            move 1 from 8 to 1
            move 10 from 1 to 9
            move 1 from 4 to 2
            move 2 from 9 to 5
            move 4 from 9 to 6
            move 1 from 2 to 7
            move 3 from 4 to 2
            move 1 from 1 to 5
            move 5 from 1 to 5
            move 1 from 4 to 9
            move 3 from 6 to 7
            move 23 from 7 to 6
            move 1 from 2 to 4
            move 1 from 2 to 5
            move 9 from 5 to 4
            move 1 from 2 to 5
            move 9 from 5 to 6
            move 1 from 9 to 7
            move 1 from 9 to 3
            move 3 from 9 to 4
            move 14 from 6 to 3
            move 5 from 7 to 4
            move 1 from 7 to 5
            move 1 from 5 to 9
            move 2 from 5 to 6
            move 16 from 6 to 2
            move 2 from 6 to 1
            move 7 from 4 to 8
            move 2 from 1 to 2
            move 4 from 3 to 5
            move 5 from 4 to 7
            move 2 from 6 to 7
            move 4 from 4 to 1
            move 4 from 8 to 9
            move 1 from 4 to 5
            move 1 from 6 to 8
            move 1 from 4 to 9
            move 4 from 1 to 7
            move 1 from 9 to 4
            move 2 from 2 to 7
            move 7 from 3 to 9
            move 15 from 2 to 3
            move 4 from 8 to 6
            move 1 from 4 to 7
            move 2 from 9 to 7
            move 1 from 6 to 8
            move 2 from 7 to 2
            move 5 from 7 to 2
            move 1 from 5 to 2
            move 6 from 2 to 9
            move 3 from 7 to 1
            move 3 from 1 to 2
            move 3 from 7 to 1
            move 2 from 2 to 9
            move 2 from 6 to 9
            move 1 from 8 to 3
            move 19 from 3 to 9
            move 1 from 6 to 3
            move 3 from 7 to 4
            move 1 from 2 to 5
            move 2 from 1 to 9
            move 2 from 2 to 3
            move 33 from 9 to 7
            move 1 from 1 to 7
            move 3 from 3 to 7
            move 1 from 3 to 2
            move 1 from 5 to 8
            move 4 from 9 to 7
            move 1 from 5 to 2
            move 2 from 4 to 9
            move 4 from 9 to 7
            move 3 from 2 to 1
            move 1 from 4 to 3
            move 1 from 9 to 7
            move 1 from 8 to 3
            move 7 from 7 to 3
            move 3 from 1 to 9
            move 4 from 9 to 7
            move 4 from 5 to 8
            move 3 from 3 to 4
            move 3 from 4 to 5
            move 3 from 3 to 6
            move 2 from 6 to 5
            move 38 from 7 to 5
            move 40 from 5 to 3
            move 4 from 8 to 9
            move 1 from 6 to 9
            move 1 from 5 to 1
            move 3 from 7 to 6
            move 1 from 7 to 5
            move 38 from 3 to 8
            move 1 from 1 to 9
            move 3 from 9 to 6
            move 5 from 3 to 9
            move 4 from 8 to 6
            move 1 from 7 to 1
            move 3 from 5 to 9
            move 1 from 1 to 2
            move 10 from 8 to 3
            move 5 from 8 to 1
            move 3 from 1 to 2
            move 9 from 6 to 7
            move 9 from 3 to 5
            move 1 from 7 to 6
            move 1 from 3 to 8
            move 1 from 7 to 9
            move 1 from 1 to 5
            move 1 from 1 to 3
            move 1 from 9 to 2
            move 4 from 2 to 3
            move 1 from 2 to 4
            move 9 from 8 to 1
            move 2 from 9 to 5
            move 2 from 1 to 2
            move 2 from 3 to 4
            move 6 from 8 to 6
            move 10 from 5 to 3
            move 7 from 3 to 2
            move 2 from 1 to 2
            move 5 from 1 to 7
            move 7 from 9 to 6
            move 7 from 6 to 5
            move 1 from 4 to 3
            move 7 from 7 to 4
            move 5 from 3 to 9
            move 7 from 2 to 6
            move 4 from 7 to 8
            move 5 from 8 to 9
            move 1 from 2 to 6
            move 1 from 3 to 5
            move 2 from 2 to 8
            move 8 from 4 to 6
            move 7 from 9 to 7
            move 4 from 7 to 9
            move 7 from 9 to 3
            move 8 from 3 to 1
            move 6 from 5 to 9
            move 8 from 1 to 8
            move 13 from 8 to 4
            move 3 from 9 to 6
            move 1 from 8 to 6
            move 1 from 7 to 3
            move 2 from 4 to 1
            move 5 from 9 to 1
            move 1 from 3 to 7
            move 15 from 6 to 1
            move 1 from 7 to 9
            move 10 from 4 to 7
            move 11 from 7 to 5
            move 17 from 1 to 6
            move 1 from 9 to 3
            move 6 from 6 to 1
            move 3 from 5 to 3
            move 2 from 4 to 5
            move 2 from 7 to 8
            move 12 from 5 to 3
            move 13 from 6 to 9
            move 2 from 8 to 2
            move 2 from 5 to 1
            move 16 from 3 to 8
            move 3 from 2 to 3
            move 2 from 3 to 7
            move 2 from 7 to 9
            move 1 from 3 to 7
            move 4 from 8 to 4
            move 2 from 4 to 8
            move 5 from 1 to 5
            move 2 from 4 to 7
            move 6 from 6 to 8
            move 2 from 8 to 5
            move 2 from 1 to 4
            move 5 from 8 to 7
            move 5 from 6 to 3
            move 6 from 9 to 8
            move 2 from 9 to 2
            move 1 from 1 to 7
            move 4 from 5 to 3
            move 2 from 2 to 3
            move 1 from 4 to 9
            move 10 from 3 to 6
            move 1 from 3 to 7
            move 10 from 7 to 2
            move 2 from 5 to 3
            move 1 from 4 to 2
            move 2 from 6 to 8
            move 3 from 6 to 5
            move 1 from 6 to 1
            move 7 from 2 to 3
            move 6 from 8 to 7
            move 4 from 6 to 3
            move 14 from 8 to 6
            move 11 from 6 to 8
            move 1 from 1 to 4
            move 6 from 7 to 2
            move 3 from 5 to 8
            move 4 from 1 to 7
            move 1 from 2 to 8
            move 1 from 2 to 6
            move 1 from 3 to 4
            move 1 from 5 to 6
            move 7 from 8 to 6
            move 9 from 3 to 2
            move 1 from 8 to 5";

        #endregion
    }
}
