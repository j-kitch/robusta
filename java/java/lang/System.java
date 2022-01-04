package java.lang;

import java.io.PrintStream;

public class System {

    public static final PrintStream out = null;

    private static native void registerNatives();

    static {
        registerNatives();
    }

    private System() {

    }
}
