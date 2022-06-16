package java.lang;

public class Throwable {

    private final Throwable cause;
    private final String message;

    public Throwable() {
        this(null, null);
    }

    public Throwable(String message) {
        this(message, null);
    }

    public Throwable(Throwable cause) {
        this(null, cause);
    }

    public Throwable(String message, Throwable cause) {
        this.message = message;
        this.cause = cause == null ? this : cause;
    }

    public String getMessage() {
        return message;
    }

    public Throwable getCause() {
        if (cause == this) {
            return null;
        }
        return cause;
    }
}
