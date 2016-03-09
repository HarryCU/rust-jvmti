import java.util.List;
import java.util.ArrayList;

public class Test {

    public static void main(final String[] args) throws Exception {
        final Test test = new Test();

//        test.printSomething();
//        test.sleepALittle();
        test.spawnThreads();
//        test.throwChecked();
//        test.throwUnchecked();
    }


    public void printSomething() {
        System.out.println("Lofasz");
    }

    public void throwChecked() {
        try {
            throw new ClassNotFoundException();
        } catch (final ClassNotFoundException e) {
            System.out.println("Caught");
        }
    }

    public void throwUnchecked() throws Exception {
        throw new NullPointerException("Application error");
    }

    public void sleepALittle() throws Exception {
        Thread.sleep(500);
        System.out.println("Slept enough");
    }

    public void spawnThreads() throws Exception {
        System.out.println("---- Spawn Threads");
        final Object object = new Object();

        final Runnable run = new Runnable() {

            public void run() {
//                System.out.println("    [" + Thread.currentThread().getName() + "] Waiting for the monitor");

                synchronized(object) {
                    System.out.println("    [" + Thread.currentThread().getName() + "] Owning the monitor, before sleep");
                    try {
                        Thread.sleep(1000);
                    } catch (Exception e) {
                    }
//                    System.out.println("    [" + Thread.currentThread().getName() + "] Owning the monitor, after sleep");
                }

//                System.out.println("    [" + Thread.currentThread().getName() + "] After acquiring the monitor");
            }
        };

        final List<Thread> threads = new ArrayList<Thread>();

        for (int i = 0; i < 4; i++) {
            final Thread thread = new Thread(run);
            threads.add(thread);
            System.out.println("Starting thread: " + thread.getName());
        }

        for (final Thread thread : threads) {
            thread.start();
        }

        for (final Thread thread : threads) {
            thread.join();
        }
    }
}
