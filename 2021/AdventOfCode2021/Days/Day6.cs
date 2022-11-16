namespace AdventOfCode2021.Days
{
    internal static class Day6
    {
        public static void Execute()
        {
            var input = Input;
            var initialState = input.Split(",").Select(int.Parse);

            // Just for the test input
            Console.WriteLine("Part 0 - " + Solve(initialState, 18));

            // How many fish do we have after 80 days?
            Console.WriteLine("Part 1 - " + Solve(initialState, 80));

            // How many fish do we have after 256 days?
            Console.WriteLine("Part 2 - " + Solve(initialState, 256));
        }

        public static long Solve(IEnumerable<int> initialState, int numDays)
        {
            var dayMap = new Dictionary<int, long>() { { 0, 0 }, { 1, 0 }, { 2, 0 }, { 3, 0 }, { 4, 0 }, { 5, 0 }, { 6, 0 }, { 7, 0 }, { 8, 0 } };

            foreach (var num in initialState)
            {
                dayMap[num]++;
            }

            for (var day = 0; day < numDays; ++day)
            {
                var fishToReset = dayMap[0];
                dayMap[0] = dayMap[1];
                dayMap[1] = dayMap[2];
                dayMap[2] = dayMap[3];
                dayMap[3] = dayMap[4];
                dayMap[4] = dayMap[5];
                dayMap[5] = dayMap[6];
                dayMap[6] = dayMap[7];
                dayMap[7] = dayMap[8];
                dayMap[8] = fishToReset;  // newly spawned fish
                dayMap[6] += fishToReset; // fish that got reset from 0 to 6
            }

            return dayMap.Values.Sum();
        }


        #region Input

        private const string TestInput = "3,4,3,1,2";

        private const string Input = "1,5,5,1,5,1,5,3,1,3,2,4,3,4,1,1,3,5,4,4,2,1,2,1,2,1,2,1,5,2,1,5,1,2,2,1,5,5,5,1,1,1,5,1,3,4,5,1,2,2,5,5,3,4,5,4,4,1,4,5,3,4,4,5,2,4,2,2,1,3,4,3,2,3,4,1,4,4,4,5,1,3,4,2,5,4,5,3,1,4,1,1,1,2,4,2,1,5,1,4,5,3,3,4,1,1,4,3,4,1,1,1,5,4,3,5,2,4,1,1,2,3,2,4,4,3,3,5,3,1,4,5,5,4,3,3,5,1,5,3,5,2,5,1,5,5,2,3,3,1,1,2,2,4,3,1,5,1,1,3,1,4,1,2,3,5,5,1,2,3,4,3,4,1,1,5,5,3,3,4,5,1,1,4,1,4,1,3,5,5,1,4,3,1,3,5,5,5,5,5,2,2,1,2,4,1,5,3,3,5,4,5,4,1,5,1,5,1,2,5,4,5,5,3,2,2,2,5,4,4,3,3,1,4,1,2,3,1,5,4,5,3,4,1,1,2,2,1,2,5,1,1,1,5,4,5,2,1,4,4,1,1,3,3,1,3,2,1,5,2,3,4,5,3,5,4,3,1,3,5,5,5,5,2,1,1,4,2,5,1,5,1,3,4,3,5,5,1,4,3\r\n";

        #endregion
    }
}
