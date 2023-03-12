package java.lang;

public class Thread {

    private static int threadInitNumber;

    private static synchronized int nextThreadNum() {
        return threadInitNumber++;
    }

    /**
     * Used internally to determine whether it's been started.
     *
     * <code>start()</code> can only be called when this == 0.
     */
    private volatile int threadStatus;
    private volatile String name;

    public Thread() {
        this("Thread-" + Integer.toString(nextThreadNum()));
    }

    public Thread(String name) {
        this.name = name;
    }

    public String getName() {
        return name;
    }

    public void run() {

    }

    public synchronized void start() {
        if (threadStatus != 0) {
            throw new RuntimeException();
        }
        nativeStart();
    }

    /**
     * Should set threadStatus.
     */
    private native void nativeStart();

    public final native void join() throws InterruptedException;

    public final native void join(long millis) throws InterruptedException;

    public static native void sleep(long millis) throws InterruptedException;

    public static native Thread currentThread();

    public native void interrupt();
}