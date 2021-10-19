// Fill out your copyright notice in the Description page of Project Settings.

using System.IO;
using UnrealBuildTool;

public class UtahTeapot : ModuleRules
{
	public UtahTeapot(ReadOnlyTargetRules Target) : base(Target)
	{
		PCHUsage = PCHUsageMode.UseExplicitOrSharedPCHs;
	
		PublicDependencyModuleNames.AddRange(new string[] { "Core", "CoreUObject", "Engine", "InputCore" });

		PrivateDependencyModuleNames.AddRange(new string[] {  });

        PublicIncludePaths.Add(Path.Combine(ModuleDirectory, "..", "ThirdParty", "UtahTeapot", "Include"));
		//PublicLibraryPaths.Add(Path.Combine(ModuleDirectory, "..", "ThirdParty", "UtahTeapot", "lib"));
        PublicAdditionalLibraries.Add(Path.Combine(ModuleDirectory, "..", "ThirdParty", "UtahTeapot", "lib", "libut.a"));
		// PublicAdditionalLibraries.Add("libstdc++");

        PublicIncludePaths.Add(Path.Combine(ModuleDirectory, "..", "ThirdParty", "Rust", "Include"));
        PublicAdditionalLibraries.Add(Path.Combine(ModuleDirectory, "..", "ThirdParty", "Rust", "target", "debug", "libextern.a"));

		// Uncomment if you are using Slate UI
		// PrivateDependencyModuleNames.AddRange(new string[] { "Slate", "SlateCore" });
		
		// Uncomment if you are using online features
		// PrivateDependencyModuleNames.Add("OnlineSubsystem");

		// To include OnlineSubsystemSteam, add it to the plugins section in your uproject file with the Enabled attribute set to true
	}
}
