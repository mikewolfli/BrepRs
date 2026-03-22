<?php
/**
 * BrepRs PHP Example
 *
 * This example demonstrates how to use BrepRs PHP bindings.
 */

echo "BrepRs PHP Example\n";
echo "==================\n";

// Initialize i18n with automatic language detection
i18n_init();
$currentLang = i18n_current_language();
echo "Current language: $currentLang\n";

// Set language to English
$result = i18n_set_language("en");
if ($result) {
    echo "Language set to English\n";
}

// Translate some messages
echo "\nTranslations (English):\n";
echo "  ErrorUnknown: " . i18n_translate("ErrorUnknown") . "\n";
echo "  LabelFile: " . i18n_translate("LabelFile") . "\n";
echo "  OpBooleanFuse: " . i18n_translate("OpBooleanFuse") . "\n";

// Set language to Simplified Chinese
$result2 = i18n_set_language("zh-CN");
if ($result2) {
    echo "\nLanguage set to Simplified Chinese\n";
}

echo "\nTranslations (Simplified Chinese):\n";
echo "  ErrorUnknown: " . i18n_translate("ErrorUnknown") . "\n";
echo "  LabelFile: " . i18n_translate("LabelFile") . "\n";
echo "  OpBooleanFuse: " . i18n_translate("OpBooleanFuse") . "\n";

// Get available languages
$languages = i18n_available_languages();
echo "\nAvailable languages: " . implode(", ", $languages) . "\n";

echo "\nExample completed successfully!\n";
?>
