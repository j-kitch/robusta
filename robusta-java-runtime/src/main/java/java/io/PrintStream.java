package java.io;

public class PrintStream {

    public native void println(boolean b);
    public native void println(char c);
    public native void println(int i);
    public native void println(long l);
    public native void println(float f);
    public native void println(double d);
    public native void println(String s);

    public void println(Object o) {
        println(String.valueOf(o));
    }
}
