import java.nio.file.Path;
import java.nio.file.Paths;

class Main {
    // This declares that the static `concat` method will be provided
    // a native library.
    private static native String concat(String word1, String word2);

    private static native double negDistance(Complex comp);

    public static void main(String[] args) {
        Path strings = Paths.get("./strings/target/debug/libstrings.so");
        System.load(strings.toAbsolutePath().toString());

        Path complexes = Paths.get("./complexes/target/debug/libcomplexes.so");
        System.load(complexes.toAbsolutePath().toString());

        String concated = Main.concat("Hello", "world");
        System.out.println(concated);
        Complex comp = new Complex(2, 2);
        double anti_dist = negDistance(comp);
        System.out.println("anti_dist = " + anti_dist);
    }
}
