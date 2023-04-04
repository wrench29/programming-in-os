import java.io.FileNotFoundException;
import java.io.FileOutputStream;
import java.io.IOException;
import java.text.SimpleDateFormat;
import java.util.Date;

public class Main {
    final static String filename = "Output.txt";
    private static FileOutputStream outputStream;
    private static volatile int counter = 0;
    private static volatile int lastPrintedCounter = 0;

    public static void main(String[] args) throws InterruptedException, FileNotFoundException {
        outputStream = new FileOutputStream(filename);

        Thread thread1 = new Thread(Main::thread1Process);
        Thread thread2 = new Thread(Main::thread2Process);
        Thread thread3 = new Thread(Main::thread3Process);

        thread1.start();
        thread2.start();
        thread3.start();
    }

    private static void thread1Process() {
        int localCounter = 0;
        while (counter <= 240) {
            writeCurrentCounter(1);

            localCounter++;
            counter = localCounter;

            try {
                Thread.sleep(250);
            } catch (InterruptedException e) {
                throw new RuntimeException(e);
            }
        }
    }

    private static void thread2Process() {
        while (counter <= 240) {
            writeCurrentCounter(2);
            try {
                Thread.sleep(500);
            } catch (InterruptedException e) {
                throw new RuntimeException(e);
            }
        }
    }

    private static void thread3Process() {
        while (counter <= 240) {
            writeCurrentCounter(3);
            try {
                Thread.sleep(1000);
            } catch (InterruptedException e) {
                throw new RuntimeException(e);
            }
        }
    }

    private static synchronized void writeCurrentCounter(int thread) {
        if (counter == lastPrintedCounter) return;

        try {
            outputStream.write(
                    String.format("Thread %d [%s] Counter: %d\n", thread, getTime(), counter)
                            .getBytes()
            );
            lastPrintedCounter = counter;
        } catch (IOException e) {
            throw new RuntimeException(e);
        }
    }

    private static String getTime() {
        return new SimpleDateFormat("HH:mm:ss").format(new Date().getTime());
    }
}