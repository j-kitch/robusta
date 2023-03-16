package java.lang;

import com.jkitch.robusta.Robusta;

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

    public void stackTraceAndExit() {
        printStackTrace();
        Robusta.exit(1);
    }

    public void printStackTrace() {
        StringBuilder message = new StringBuilder();
        message.append("Exception in thread \"?\" " + getClass().getName() + ": " + getMessage() + "\n");
        for (StackTraceElement element : stackTrace) {
            message.append("\t" + element.toString() + "\n");
        }
        String result = message.toString();
        Robusta.printerr(result);
    }
}