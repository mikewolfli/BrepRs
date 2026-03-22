/**
 * BrepRs Node.js Example
 *
 * This example demonstrates how to use BrepRs Node.js bindings.
 */

const breprs = require('breprs');

function main() {
    console.log('BrepRs Node.js Example');
    console.log('======================');

    // Initialize i18n with automatic language detection
    breprs.i18nInit();
    const currentLang = breprs.i18nCurrentLanguage();
    console.log(`Current language: ${currentLang}`);

    // Set language to English
    const result = breprs.i18nSetLanguage('en');
    if (result) {
        console.log('Language set to English');
    }

    // Translate some messages
    console.log('\nTranslations (English):');
    console.log(`  ErrorUnknown: ${breprs.i18nTranslate('ErrorUnknown')}`);
    console.log(`  LabelFile: ${breprs.i18nTranslate('LabelFile')}`);
    console.log(`  OpBooleanFuse: ${breprs.i18nTranslate('OpBooleanFuse')}`);

    // Set language to Simplified Chinese
    const result2 = breprs.i18nSetLanguage('zh-CN');
    if (result2) {
        console.log('\nLanguage set to Simplified Chinese');
    }

    console.log('\nTranslations (Simplified Chinese):');
    console.log(`  ErrorUnknown: ${breprs.i18nTranslate('ErrorUnknown')}`);
    console.log(`  LabelFile: ${breprs.i18nTranslate('LabelFile')}`);
    console.log(`  OpBooleanFuse: ${breprs.i18nTranslate('OpBooleanFuse')}`);

    // Get available languages
    const languages = breprs.i18nAvailableLanguages();
    console.log(`\nAvailable languages: ${languages.join(', ')}`);

    console.log('\nExample completed successfully!');
}

main();
