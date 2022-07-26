workflow "BUILD" {
    on = "EVENT"
    resolves = "DEPLOY"
}

workflow "DEPLOY" {
    needs = "BUILD"
}