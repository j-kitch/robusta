package java.lang;

public class Throwable {

    private String detailMessage;
    private StackTraceElement[] stackTrace = new StackTraceElement[0];

    public Throwable() {
        fillInStackTrace();
    }

    public Throwable(String detailMessage) {
        this();
        this.detailMessage = detailMessage;
    }

    public String getMessage() {
        return detailMessage;
    }

    private native void fillInStackTrace();

    public native void printStackTrace();
}