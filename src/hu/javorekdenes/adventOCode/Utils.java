package hu.javorekdenes.adventOCode;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;

public class Utils {
    public static List<String> getStringsFromFile(String filename) {
        List<String> lines;
        try {
            lines = Files.readAllLines(Paths.get(filename));
            String[] arr = lines.toArray(new String[0]);
            return new ArrayList<>(Arrays.asList(arr));
        } catch (IOException e) {
            e.printStackTrace();
        }

        return List.of();
    }

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
