package java.lang;

public final class Class<T> {

    private final String name;

    private Class(String name) {
        this.name = name;
    }

    public String getName() {
        return name;
    }
}