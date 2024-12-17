plugins {
    kotlin("jvm") version "1.8.10"
    application
    id("org.openjfx.javafxplugin") version "0.0.14" // Add JavaFX plugin
}

repositories {
    mavenCentral() // Use Maven Central to fetch dependencies
}

dependencies {
    val javafxVersion = "20.0.2" // Compatible with JDK 19
    implementation("org.openjfx:javafx-controls:$javafxVersion")
    implementation("org.openjfx:javafx-fxml:$javafxVersion")
    implementation("org.openjfx:javafx-graphics:$javafxVersion")
}

java {
    toolchain {
        // Set Java language version to JDK 19
        languageVersion.set(JavaLanguageVersion.of(19))
    }
}

application {
    // Specify the main class path
    mainClass.set("com.nenuadrian.Main")
}

javafx {
    // JavaFX plugin configuration
    version = "20.0.2"
    modules = listOf("javafx.controls", "javafx.fxml", "javafx.graphics")
}
