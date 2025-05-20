# desecret
Remove base64 encoding from a kubernetes secret, showing secrets in clear text

## motivation

Sometime you need to check out quickly some settings that are stored in a kubernetes secret.

Those secrets can be defined using the `dataString` map but they are always output as raw `data` with a base64 encoding.
This can be annoying to checkout and doesn't offer any additional security.
Sure, you can always fiddle with jsonpath, jq, yq and/or base64 and get the value you need, but I wanted something
simple to peek the secret content.

## Usage

`desecret` is used in pipelines: it trasform a secret `data` map from a yaml structure into a `stringData`, like this:

```sh
kubectl get secret my-secret -o yaml|desecret
```


### example
```sh
kubectl get secret my-secret -o yaml
```
```yaml
apiVersion: v1
data:
  database: cG9zdGdyZXM=
  password: cG9zdGdyZXNwYXNzd29yZA==
  url: cG9zdGdyZXNxbDovL3Bvc3RncmVzOnBvc3RncmVzcGFzc3dvcmRAcG9zdGdyZXMuZGVmYXVsdC5zdmMuY2x1c3Rlci5sb2NhbDo1NDMyL3Bvc3RncmVz
  username: cG9zdGdyZXM=
kind: Secret
metadata:
  creationTimestamp: "2024-09-03T10:28:09Z"
  name: my-secret
  namespace: default
  resourceVersion: "342013"
  uid: 85c55e2d-ee2f-4e94-a1d4-ad65dd23ae94
type: Opaque
```

```sh
kubectl get secret my-secret -o yaml|desecret
```
```yaml
apiVersion: v1
kind: Secret
metadata:
  creationTimestamp: "2024-09-03T10:28:09Z"
  name: my-secret
  namespace: default
  resourceVersion: "342013"
  uid: 85c55e2d-ee2f-4e94-a1d4-ad65dd23ae94
type: Opaque
stringData:
  database: postgres
  password: postgrespassword
  url: "postgresql://postgres:postgrespassword@postgres.default.svc.cluster.local:5432/postgres"
  username: postgres
```
