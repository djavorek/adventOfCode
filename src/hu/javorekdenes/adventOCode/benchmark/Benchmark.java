package hu.javorekdenes.adventOCode.benchmark;

public class Benchmark {
    // TODO: Check again
    // https://www.oracle.com/technical-resources/articles/java/architect-benchmarking.html
    public static void bench(String name, long runMillis, int loop,
                             int warmupIterations, int actualIterations, Runnable runnable) {
        System.out.println("Running benchmark: " + name);
        int totalIterations = actualIterations + warmupIterations;
        long totalThroughput = 0L;
        long average;
        for (int i = 0; i < totalIterations; i++) {
            long count = 0;
            long duration = 0L;
            long start = System.currentTimeMillis();
            while (duration < runMillis) {
                for (int j = 0; j < loop; j++) {
                    runnable.run();
                    count++;
                }
                duration = System.currentTimeMillis() - start;
            }
            long throughput = count / duration;
            boolean actualRun = i >= warmupIterations;
            if (actualRun) {
                totalThroughput += throughput;
            }
            System.out.print(throughput + " ops/ms" + (!actualRun ? " (warmup) | " : " | "));
        }
        average = totalThroughput / actualIterations;
        System.out.println();
        System.out.println("[ ~" + average + " ops/ms ]");
    }
}
