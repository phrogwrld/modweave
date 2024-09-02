plugins {
    id("fabric-loom") version "1.7-SNAPSHOT"
    id("maven-publish")
}

group = project.property("maven_group") as String
version = project.property("mod_version") as String

repositories {
    // Add repositories here
}

dependencies {
    minecraft("com.mojang:minecraft:${project.property("minecraft_version")}")
    mappings("net.fabricmc:yarn:${project.property("yarn_mappings")}:v2")
    modImplementation("net.fabricmc:fabric-loader:${project.property("fabric_loader_version")}")

    modImplementation("net.fabricmc.fabric-api:fabric-api:${project.property("fabric_api_version")}")
}

tasks {
    processResources {
        inputs.property("version", project.version)
        filteringCharset = "UTF-8"

        filesMatching("fabric.mod.json") {
            expand(mutableMapOf("version" to project.version))
        }
    }

    jar {
        from("LICENSE") {
            rename { "${it}_${project.property("archivesBaseName")}" }
        }
    }
}

java {
    toolchain {
        languageVersion.set(JavaLanguageVersion.of(${{ java_version }}))
    }
    withSourcesJar()
    sourceCompatibility = JavaVersion.VERSION_${{ java_version }}
    targetCompatibility = JavaVersion.VERSION_${{ java_version }}
}

tasks.withType<JavaCompile> {
    options.encoding = "UTF-8"
    options.release.set(${{ java_version }})
}