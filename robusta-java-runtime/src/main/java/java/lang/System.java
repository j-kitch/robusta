package java.lang;

import java.io.PrintStream;

public class System {

    public static final PrintStream out = new PrintStream();

    private System() {

    }

    public static native void arraycopy(Object src, int srcPos,
                                        Object dest, int destPos,
                                        int length);
}
