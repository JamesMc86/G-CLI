pipeline {
agent none

	stages {
		stage("VS Building") {
			agent {label 'VS16'}
			stages {
				stage ('Get VS Dependencies') {	
					steps {
						bat 'nuget restore \"C Sharp Source/LabVIEW CLI/G CLI.sln\"'
					}
				}

				stage ('VS Build') {
					steps {
						changeAsmVer versionPattern: "${env.BUILD_NUMBER}", regexPattern: "Assembly(\\w*)Version\\(\"(\\d+).(\\d+).(\\d+).(\\d+)\"\\)", replacementPattern:"Assembly\$1Version(\"\$2.\$3.\$4.%s\")"
						bat "\"${tool 'MSBuild-16'}\" \"C Sharp Source/LabVIEW CLI/G CLI.sln\" /p:Configuration=Release /p:Platform=\"x86\""
						bat "\"${tool 'MSBuild-16'}\" \"C Sharp Source/LabVIEW CLI/G CLI.sln\" /p:Configuration=Release /p:Platform=\"x64\""
						bat "MoveInstallers.bat"
						stash name:"installers", includes:"LabVIEW Source/Installation Support/*.msi"
					}
				}
					
				stage ('VS Test'){
					steps{
						bat "if exist TestResults/VSTestResults.trx del TestResults/VSTestResults.trx"
						bat "\"${tool 'VSTest-16'}\" \"%WORKSPACE%/C Sharp Source/LabVIEWCLI_Unit_tests/bin/x86/Release/GCLI_Unit_tests.dll\" --logger:trx;LogFileName=VSTestResults.trx"
						step([$class: 'MSTestPublisher', testResultsFile:"TestResults/VSTestResults.trx", failOnError: true, keepLongStdio: true])
					}
				}
				
					
				stage ('VS Integration Test') {
					steps {
						bat 'pushd \"Integration Tests\" & \"Run Integration Tests.bat\" \"../C Sharp Source/LabVIEW CLI/bin/x64/Release/" & popd'
					}
				}
			}
		}
			
		stage ('LabVIEW Builds') {
			agent {label 'LV2011'}
			stages {
				stage ('Get Dependencies') {
					steps {
						unstash 'installers'
						bat "labview-cli -v \"C:\\Users\\Public\\Documents\\National Instruments\\LV-CLI Common Steps\\steps\\vipcApply.vi\" -- \"${env.WORKSPACE}\\LabVIEW Source\\Dependencies\\G CLI Dev Dependencies.vipc\" 2011"
					}
				}
				
				/* removed due to lack of junit support in lv2011
				stage ('Unit Testing') {
					steps {
						bat "labview-cli -v \"C:\\Users\\Public\\Documents\\National Instruments\\LV-CLI Common Steps\\steps\\run-vi-tester.vi\" -- \"LabVIEW Source\\G-CLI.lvproj\" \"lv-results.xml\" \"${env.WORKSPACE}\" "
						junit "lv-results.xml"
					}
				}
				*/
				
				stage ('LabVIEW Build') {
					steps {
						bat "labview-cli -v --kill \"C:\\Users\\Public\\Documents\\National Instruments\\LV-CLI Common Steps\\steps\\setVipBuildNumber.vi\" -- \"LabVIEW Source\\G CLI.vipb\" \"${env.WORKSPACE}\" ${env.BUILD_NUMBER}"
						bat "if not exist Builds mkdir Builds"
						//call direct as build fails if CLI toolkit is already loaded.
						bat "\"C:\\Program Files (x86)\\National Instruments\\LabVIEW 2011\\LabVIEW.exe\" \"C:\\Users\\Public\\Documents\\National Instruments\\LV-CLI Common Steps\\steps\\vipbBuild-nocli.vi\" -- \"LabVIEW Source\\G CLI.vipb\" Builds  \"${env.WORKSPACE}\""
					}
				}
			
			}
			
			post {
				always {
					archiveArtifacts artifacts:"LabVIEW Source/Installation Support/*.msi", fingerprint: true
					dir ("Builds") {
						archiveArtifacts artifacts: '*.vip', fingerprint: true
						deleteDir()
					}

				}
			}
		}
		

	}
}
