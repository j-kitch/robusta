package java.lang;

public final class StringBuilder {

    private char[] chars;
    private int count;

    public StringBuilder() {
        chars = new char[0];
        count = 0;
    }

    public StringBuilder append(String str) {
        int newCount = count + str.length();

        if (newCount > chars.length) {
            char[] newChars = new char[newCount];
            for (int i = 0; i < count; i++) {
                newChars[i] = chars[i];
            }
            chars = newChars;
        }

        char[] nextChars = str.toCharArray();
        for (int i = 0; i < str.length(); i++) {
            chars[count] = nextChars[i];
            count += 1;
        }
        return this;
    }
}