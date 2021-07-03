using System;
using System.Collections.Generic;
using System.Text;


namespace game
{
    public class Bot
    {

        private readonly Dictionary<(UInt64, int), double> states = new();
        private Random random = new Random();
        public int intersections = 0;


        private readonly double explorationRateDecay = 0.009;

        /// Rate at witch the Q is changed per step
        private readonly double learningRate = 0.05;

        /// Degradation rate of future rewards
        private readonly double discountRate = 0.95;

        public Bot()
        {
        }

        public void Learn(int nEpisodes)
        {
            var currentPercent = 0;
            //var episoedPercentage = 100d / nEpisodes;

            var explorationRate = 1d;

            var scoreSum = 0;

            for (var i = 0; i < nEpisodes; i++)
            {
                scoreSum += PlayOneGame(explorationRate);

                if (currentPercent < i * 100d / nEpisodes)
                {
                    explorationRate -= explorationRateDecay;
                    currentPercent++;
                    var avg = scoreSum / (nEpisodes / 100d);
                    scoreSum = 0;
                    Console.WriteLine($"perc: {currentPercent}%    step: {i}    explorationRate: {explorationRate}    AverageScore: {avg}");
                }
            }
        }

        public int PlayOneGame(double explorationRate)
        {
            var gameInstance = new GameInstance(4);
            var moves = gameInstance.PossibleMoves();

            var gameState = gameInstance.GetState();

            do
            {
                var explor = random.NextDouble() < explorationRate;

                int dir;
                if (explor)
                {
                    var pos = random.Next(moves.Count);
                    dir = moves[pos];
                }
                else
                {
                    dir = bestMove(moves, gameState);
                }

                var reward = gameInstance.Play(dir);

                var nextGameState = gameInstance.GetState();
                var nextMoves = gameInstance.PossibleMoves();
                var nextBestDir = bestMove(moves, nextGameState);

                states.TryGetValue((nextGameState, nextBestDir), out var nextMaxQ);
                states.TryGetValue((gameState, dir), out var q);

                if (nextMoves.Count == 0)
                {
                    reward = -20;
                }

                states[(gameState, dir)] = (1 - learningRate) * q + learningRate * (reward + discountRate * nextMaxQ);

                moves = nextMoves;
                gameState = nextGameState;

            } while (moves.Count != 0);

            return gameInstance.FinalScore();

            //Console.WriteLine(finalScore);

        }

        private int bestMove(List<int> moves, ulong gameState)
        {
            int dir;
            var bestScore = double.MinValue;
            dir = moves[0];
            foreach (var move in moves)
            {
                states.TryGetValue((gameState, move), out var score);
                if (bestScore < score)
                {
                    bestScore = score;
                    dir = move;
                }
            }

            return dir;
        }
    }
}
