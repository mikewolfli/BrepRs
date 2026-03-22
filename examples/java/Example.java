import com.breprs.I18n;

/**
 * BrepRs Java Example
 *
 * This example demonstrates how to use BrepRs Java bindings.
 */
public class Example {
    public static void main(String[] args) {
        System.out.println("BrepRs Java Example");
        System.out.println("===================");

        // Initialize i18n with automatic language detection
        I18n.init();
        String currentLang = I18n.currentLanguage();
        System.out.println("Current language: " + currentLang);

        // Set language to English
        int result = I18n.setLanguage("en");
        if (result == 0) {
            System.out.println("Language set to English");
        }

        // Translate some messages
        System.out.println("\nTranslations (English):");
        System.out.println("  ErrorUnknown: " + I18n.translate("ErrorUnknown"));
        System.out.println("  LabelFile: " + I18n.translate("LabelFile"));
        System.out.println("  OpBooleanFuse: " + I18n.translate("OpBooleanFuse"));

        // Set language to Simplified Chinese
        result = I18n.setLanguage("zh-CN");
        if (result == 0) {
            System.out.println("\nLanguage set to Simplified Chinese");
        }

        System.out.println("\nTranslations (Simplified Chinese):");
        System.out.println("  ErrorUnknown: " + I18n.translate("ErrorUnknown"));
        System.out.println("  LabelFile: " + I18n.translate("LabelFile"));
        System.out.println("  OpBooleanFuse: " + I18n.translate("OpBooleanFuse"));

        System.out.println("\nExample completed successfully!");
    }
}
