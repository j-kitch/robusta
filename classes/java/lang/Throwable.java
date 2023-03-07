package java.lang;

public class Throwable {

    private String detailMessage;

    public Throwable() {
    }

    public Throwable(String detailMessage) {
        this.detailMessage = detailMessage;
    }

    public String getMessage() {
        return detailMessage;
    }
}