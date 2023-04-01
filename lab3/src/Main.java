import java.util.Scanner;
import java.util.concurrent.*;

public class Main {
    public static void main(String[] args) throws InterruptedException, ExecutionException {
        Scanner scanner = new Scanner(System.in);
        System.out.print("Enter index of fibonacci number: ");
        long index = scanner.nextLong();

        System.out.println("Waiting for async fib(" + index + ") function");
        Future<Long> resultFuture = calculateAsync(index);
        while (!resultFuture.isDone()) {
            System.out.println("Waiting...");
            Thread.sleep(200);
        }
        long result = resultFuture.get();
        System.out.println("Result: " + result);
    }

    public static Future<Long> calculateAsync(long index) {
        CompletableFuture<Long> completableFuture = new CompletableFuture<>();

        ExecutorService pool = Executors.newCachedThreadPool();
        pool.submit(() -> {
            completableFuture.complete(fib(index));
            return null;
        });
        pool.shutdown();

        return completableFuture;
    }

    public static Long fib(Long n)
    {
        if (n <= 1) {
            return n;
        }

        return fib(n - 1) + fib(n - 2);
    }
}