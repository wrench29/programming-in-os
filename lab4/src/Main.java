import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import java.util.Random;
import java.util.stream.IntStream;

public class Main {
    static final int ARRAY_SIZE = 10000;

    /*
    Asynchronous execution of an easy task will
    take longer than synchronous. An asynchronous
    execution of a heavy task will run much faster
    than a synchronous one.
    */
    public static void main(String[] args) {
        runSyncVariant();
        runSyncVariantWithSleep();
        runAsyncVariant();
        runAsyncVariantWithSleep();
    }

    private static void runSyncVariantWithSleep() {
        System.out.println("Start sync process with sleep");

        long startTime = System.nanoTime();
        int[] result = syncVariant(true);
        long endTime = System.nanoTime();

        double seconds = (double)(endTime - startTime) / 1_000_000_000;
        System.out.printf("Duration: %.3f seconds\n", seconds);
    }

    private static void runSyncVariant() {
        System.out.println("Start sync process without sleep");

        long startTime = System.nanoTime();
        int[] result = syncVariant(false);
        long endTime = System.nanoTime();

        double seconds = (double)(endTime - startTime) / 1_000_000_000;
        System.out.printf("Duration: %.3f seconds\n", seconds);
    }

    private static void runAsyncVariantWithSleep() {
        System.out.println("Start async process with sleep");

        long startTime = System.nanoTime();
        int[] result = asyncVariant(true);
        long endTime = System.nanoTime();

        double seconds = (double)(endTime - startTime) / 1_000_000_000;
        System.out.printf("Duration: %.3f seconds\n", seconds);
    }

    private static void runAsyncVariant() {
        System.out.println("Start async process without sleep");

        long startTime = System.nanoTime();
        int[] result = asyncVariant(false);
        long endTime = System.nanoTime();

        double seconds = (double)(endTime - startTime) / 1_000_000_000;
        System.out.printf("Duration: %.3f seconds\n", seconds);
    }

    private static int[] syncVariant(boolean doSleep) {
        int[] arrayA = generateArray(ARRAY_SIZE);
        int[] arrayB = generateArray(ARRAY_SIZE);

        int[] outputArray = new int[ARRAY_SIZE];

        for (int i = 0; i < ARRAY_SIZE; i++) {
            outputArray[i] = arrayA[i] * arrayB[i];
            safeSleep1ms(doSleep);
        }

        return outputArray;
    }

    private static int[] asyncVariant(boolean doSleep) {
        int[] arrayA = generateArray(ARRAY_SIZE);
        int[] arrayB = generateArray(ARRAY_SIZE);

        int[] outputArray = new int[ARRAY_SIZE];

        IntStream.range(0, ARRAY_SIZE).parallel().forEach(i -> {
            outputArray[i] = arrayA[i] * arrayB[i];
            safeSleep1ms(doSleep);
        });

        return outputArray;
    }

    private static void safeSleep1ms(boolean doSleep) {
        if (!doSleep) {
            return;
        }

        try {
            Thread.sleep(1);
        } catch (InterruptedException e) {
            // Ok
        }
    }

    private static int[] generateArray(int size) {
        int[] array = new int[size];
        Random rand = new Random();

        for (int i = 0; i < size; i++) {
            array[i] = rand.nextInt(100);
        }

        return array;
    }
}
