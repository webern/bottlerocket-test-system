# Bottlerocket Test System Sample Files

These files contain templates that can be populated using `sed` commands and run with the `cli` tool.

Examples of how to populate each one of these files can be found below.

Each codeblock starts with a small number of variables that need to be populated before running the block, followed by a number of variables that contain default values (but can be modified by the user).

_Note_: `ASSUME_ROLE` has a default value of `~` (null), but you can replace this with the ARN of an AWS IAM role that should be used for all AWS calls.

The final `cat` command will print the populated file to the path indicated by `OUTPUT_FILE`.

## EKS

The files in [eks](./eks) are meant to be run on an EKS test cluster. You can create a new cluster using the [eksctl](https://eksctl.io/introduction/) tool.

### Migration Testing on `aws-ecs` Variants

```bash
export CLUSTER_NAME="x86-64-aws-ecs-1"
export OUTPUT_FILE="${CLUSTER_NAME}-migration.yaml"
export VARIANT="aws-ecs-1"
export ARCHITECTURE="x86_64"
export AGENT_IMAGE_VERSION=$(cli --version | sed -e "s/^.* //g")
export ECS_TEST_AGENT_IMAGE_URI="public.ecr.aws/bottlerocket-test-system/ecs-test-agent:v${AGENT_IMAGE_VERSION}"
export MIGRATION_TEST_AGENT_IMAGE_URI="public.ecr.aws/bottlerocket-test-system/migration-test-agent:v${AGENT_IMAGE_VERSION}"
export ECS_RESOURCE_AGENT_IMAGE_URI="public.ecr.aws/bottlerocket-test-system/ecs-resource-agent:v${AGENT_IMAGE_VERSION}"
export EC2_RESOURCE_AGENT_IMAGE_URI="public.ecr.aws/bottlerocket-test-system/ec2-resource-agent:v${AGENT_IMAGE_VERSION}"
export ASSUME_ROLE="~"
export AWS_REGION="us-west-2"
export UPGRADE_VERSION="v1.11.1"
export STARTING_VERSION="v1.11.0"
export METADATA_URL="https://updates.bottlerocket.aws/2020-07-07/${VARIANT}/${ARCHITECTURE}"
export TARGETS_URL="https://updates.bottlerocket.aws/targets"

export BOTTLEROCKET_AMI_ID=$(aws ssm get-parameter \
  --region ${AWS_REGION} \
  --name "/aws/service/bottlerocket/${VARIANT}/${ARCHITECTURE}/${STARTING_VERSION}/image_id" \
  --query Parameter.Value --output text)

cat eks/ecs-migration-test.yaml \
  | sed -e "s/<CLUSTER-NAME>/$(echo ${CLUSTER_NAME} | sed 's/[^a-zA-Z 0-9]/\\&/g')/g" \
  | sed -e "s/<ECS-TEST-AGENT-IMAGE-URI>/$(echo ${ECS_TEST_AGENT_IMAGE_URI} | sed 's/[^a-zA-Z 0-9]/\\&/g')/g" \
  | sed -e "s/<ASSUME-ROLE>/$(echo ${ASSUME_ROLE} | sed 's/[^a-zA-Z 0-9]/\\&/g')/g" \
  | sed -e "s/<AWS-REGION>/$(echo ${AWS_REGION} | sed 's/[^a-zA-Z 0-9]/\\&/g')/g" \
  | sed -e "s/<UPGRADE-VERSION>/$(echo ${UPGRADE_VERSION} | sed 's/[^a-zA-Z 0-9]/\\&/g')/g" \
  | sed -e "s/<STARTING-VERSION>/$(echo ${STARTING_VERSION} | sed 's/[^a-zA-Z 0-9]/\\&/g')/g" \
  | sed -e "s/<METADATA-URL>/$(echo ${METADATA_URL} | sed 's/[^a-zA-Z 0-9]/\\&/g')/g" \
  | sed -e "s/<TARGETS-URL>/$(echo ${TARGETS_URL} | sed 's/[^a-zA-Z 0-9]/\\&/g')/g" \
  | sed -e "s/<MIGRATION-TEST-AGENT-IMAGE-URI>/$(echo ${MIGRATION_TEST_AGENT_IMAGE_URI} | sed 's/[^a-zA-Z 0-9]/\\&/g')/g" \
  | sed -e "s/<ECS-RESOURCE-AGENT-IMAGE-URI>/$(echo ${ECS_RESOURCE_AGENT_IMAGE_URI} | sed 's/[^a-zA-Z 0-9]/\\&/g')/g" \
  | sed -e "s/<EC2-RESOURCE-AGENT-IMAGE-URI>/$(echo ${EC2_RESOURCE_AGENT_IMAGE_URI} | sed 's/[^a-zA-Z 0-9]/\\&/g')/g" \
  | sed -e "s/<BOTTLEROCKET-AMI-ID>/${BOTTLEROCKET_AMI_ID}/g" \
  > ${OUTPUT_FILE}
```

### Conformance Testing on `aws-ecs` Variants

```bash
export CLUSTER_NAME="x86-64-aws-ecs-1"
export OUTPUT_FILE="${CLUSTER_NAME}.yaml"
export VARIANT="aws-ecs-1"
export ARCHITECTURE="x86_64"
export AGENT_IMAGE_VERSION=$(cli --version | sed -e "s/^.* //g")
export ECS_RESOURCE_AGENT_IMAGE_URI="public.ecr.aws/bottlerocket-test-system/ecs-resource-agent:v${AGENT_IMAGE_VERSION}"
export EC2_RESOURCE_AGENT_IMAGE_URI="public.ecr.aws/bottlerocket-test-system/ec2-resource-agent:v${AGENT_IMAGE_VERSION}"
export ECS_TEST_AGENT_IMAGE_URI="public.ecr.aws/bottlerocket-test-system/ecs-test-agent:v${AGENT_IMAGE_VERSION}"
export ASSUME_ROLE="~"
export AWS_REGION="us-west-2"

export BOTTLEROCKET_AMI_ID=$(aws ssm get-parameter \
  --region ${AWS_REGION} \
  --name "/aws/service/bottlerocket/${VARIANT}/${ARCHITECTURE}/latest/image_id" \
  --query Parameter.Value --output text)

cat eks/ecs-test.yaml \
  | sed -e "s/<CLUSTER-NAME>/$(echo ${CLUSTER_NAME} | sed 's/[^a-zA-Z 0-9]/\\&/g')/g" \
  | sed -e "s/<ECS-TEST-AGENT-IMAGE-URI>/$(echo ${ECS_TEST_AGENT_IMAGE_URI} | sed 's/[^a-zA-Z 0-9]/\\&/g')/g" \
  | sed -e "s/<ASSUME-ROLE>/$(echo ${ASSUME_ROLE} | sed 's/[^a-zA-Z 0-9]/\\&/g')/g" \
  | sed -e "s/<AWS-REGION>/$(echo ${AWS_REGION} | sed 's/[^a-zA-Z 0-9]/\\&/g')/g" \
  | sed -e "s/<ECS-RESOURCE-AGENT-IMAGE-URI>/$(echo ${ECS_RESOURCE_AGENT_IMAGE_URI} | sed 's/[^a-zA-Z 0-9]/\\&/g')/g" \
  | sed -e "s/<EC2-RESOURCE-AGENT-IMAGE-URI>/$(echo ${EC2_RESOURCE_AGENT_IMAGE_URI} | sed 's/[^a-zA-Z 0-9]/\\&/g')/g" \
  | sed -e "s/<BOTTLEROCKET-AMI-ID>/${BOTTLEROCKET_AMI_ID}/g" \
  > ${OUTPUT_FILE}
```

### Migration Testing on `aws-k8s` Variants

```bash
export CLUSTER_NAME="x86-64-aws-k8s-124"
export OUTPUT_FILE="${CLUSTER_NAME}-migration.yaml"
export VARIANT="aws-k8s-1.24"
export ARCHITECTURE="x86_64"
export AGENT_IMAGE_VERSION=$(cli --version | sed -e "s/^.* //g")
export SONOBUOY_TEST_AGENT_IMAGE_URI="public.ecr.aws/bottlerocket-test-system/sonobuoy-test-agent:v${AGENT_IMAGE_VERSION}"
export MIGRATION_TEST_AGENT_IMAGE_URI="public.ecr.aws/bottlerocket-test-system/migration-test-agent:v${AGENT_IMAGE_VERSION}"
export EKS_RESOURCE_AGENT_IMAGE_URI="public.ecr.aws/bottlerocket-test-system/eks-resource-agent:v${AGENT_IMAGE_VERSION}"
export EC2_RESOURCE_AGENT_IMAGE_URI="public.ecr.aws/bottlerocket-test-system/ec2-resource-agent:v${AGENT_IMAGE_VERSION}"
export ASSUME_ROLE="~"
export AWS_REGION="us-west-2"
export UPGRADE_VERSION="v1.11.1"
export STARTING_VERSION="v1.11.0"
export METADATA_URL="https://updates.bottlerocket.aws/2020-07-07/${VARIANT}/${ARCHITECTURE}"
export TARGETS_URL="https://updates.bottlerocket.aws/targets"

export BOTTLEROCKET_AMI_ID=$(aws ssm get-parameter \
  --region ${AWS_REGION} \
  --name "/aws/service/bottlerocket/${VARIANT}/${ARCHITECTURE}/${STARTING_VERSION}/image_id" \
  --query Parameter.Value --output text)

cat eks/sonobuoy-migration-test.yaml \
  | sed -e "s/<CLUSTER-NAME>/$(echo ${CLUSTER_NAME} | sed 's/[^a-zA-Z 0-9]/\\&/g')/g" \
  | sed -e "s/<SONOBUOY-TEST-AGENT-IMAGE-URI>/$(echo ${SONOBUOY_TEST_AGENT_IMAGE_URI} | sed 's/[^a-zA-Z 0-9]/\\&/g')/g" \
  | sed -e "s/<ASSUME-ROLE>/$(echo ${ASSUME_ROLE} | sed 's/[^a-zA-Z 0-9]/\\&/g')/g" \
  | sed -e "s/<AWS-REGION>/$(echo ${AWS_REGION} | sed 's/[^a-zA-Z 0-9]/\\&/g')/g" \
  | sed -e "s/<UPGRADE-VERSION>/$(echo ${UPGRADE_VERSION} | sed 's/[^a-zA-Z 0-9]/\\&/g')/g" \
  | sed -e "s/<STARTING-VERSION>/$(echo ${STARTING_VERSION} | sed 's/[^a-zA-Z 0-9]/\\&/g')/g" \
  | sed -e "s/<METADATA-URL>/$(echo ${METADATA_URL} | sed 's/[^a-zA-Z 0-9]/\\&/g')/g" \
  | sed -e "s/<TARGETS-URL>/$(echo ${TARGETS_URL} | sed 's/[^a-zA-Z 0-9]/\\&/g')/g" \
  | sed -e "s/<MIGRATION-TEST-AGENT-IMAGE-URI>/$(echo ${MIGRATION_TEST_AGENT_IMAGE_URI} | sed 's/[^a-zA-Z 0-9]/\\&/g')/g" \
  | sed -e "s/<EKS-RESOURCE-AGENT-IMAGE-URI>/$(echo ${EKS_RESOURCE_AGENT_IMAGE_URI} | sed 's/[^a-zA-Z 0-9]/\\&/g')/g" \
  | sed -e "s/<EC2-RESOURCE-AGENT-IMAGE-URI>/$(echo ${EC2_RESOURCE_AGENT_IMAGE_URI} | sed 's/[^a-zA-Z 0-9]/\\&/g')/g" \
  | sed -e "s/<BOTTLEROCKET-AMI-ID>/${BOTTLEROCKET_AMI_ID}/g" \
  > ${OUTPUT_FILE}
```

### Conformance Testing on `aws-k8s` Variants

```bash
export CLUSTER_NAME="x86-64-aws-k8s-124"
export OUTPUT_FILE="${CLUSTER_NAME}.yaml"
export VARIANT="aws-k8s-1.24"
export ARCHITECTURE="x86_64"
export AGENT_IMAGE_VERSION=$(cli --version | sed -e "s/^.* //g")
export SONOBUOY_TEST_AGENT_IMAGE_URI="public.ecr.aws/bottlerocket-test-system/sonobuoy-test-agent:v${AGENT_IMAGE_VERSION}"
export EKS_RESOURCE_AGENT_IMAGE_URI="public.ecr.aws/bottlerocket-test-system/eks-resource-agent:v${AGENT_IMAGE_VERSION}"
export EC2_RESOURCE_AGENT_IMAGE_URI="public.ecr.aws/bottlerocket-test-system/ec2-resource-agent:v${AGENT_IMAGE_VERSION}"
export ASSUME_ROLE="~"
export AWS_REGION="us-west-2"
export SONOBUOY_MODE="quick"

export BOTTLEROCKET_AMI_ID=$(aws ssm get-parameter \
  --region ${AWS_REGION} \
  --name "/aws/service/bottlerocket/${VARIANT}/${ARCHITECTURE}/latest/image_id" \
  --query Parameter.Value --output text)

cat eks/sonobuoy-test.yaml \
  | sed -e "s/<CLUSTER-NAME>/$(echo ${CLUSTER_NAME} | sed 's/[^a-zA-Z 0-9]/\\&/g')/g" \
  | sed -e "s/<SONOBUOY-TEST-AGENT-IMAGE-URI>/$(echo ${SONOBUOY_TEST_AGENT_IMAGE_URI} | sed 's/[^a-zA-Z 0-9]/\\&/g')/g" \
  | sed -e "s/<ASSUME-ROLE>/$(echo ${ASSUME_ROLE} | sed 's/[^a-zA-Z 0-9]/\\&/g')/g" \
  | sed -e "s/<AWS-REGION>/$(echo ${AWS_REGION} | sed 's/[^a-zA-Z 0-9]/\\&/g')/g" \
  | sed -e "s/<SONOBUOY-MODE>/$(echo ${SONOBUOY_MODE} | sed 's/[^a-zA-Z 0-9]/\\&/g')/g" \
  | sed -e "s/<EKS-RESOURCE-AGENT-IMAGE-URI>/$(echo ${EKS_RESOURCE_AGENT_IMAGE_URI} | sed 's/[^a-zA-Z 0-9]/\\&/g')/g" \
  | sed -e "s/<EC2-RESOURCE-AGENT-IMAGE-URI>/$(echo ${EC2_RESOURCE_AGENT_IMAGE_URI} | sed 's/[^a-zA-Z 0-9]/\\&/g')/g" \
  | sed -e "s/<BOTTLEROCKET-AMI-ID>/${BOTTLEROCKET_AMI_ID}/g" \
  > ${OUTPUT_FILE}
```

### Migration Testing on `vmware-k8s` Variants

This codeblock assumes that your vSphere config file has been sourced. Specifically, the variables `GOVC_USERNAME`, `GOVC_PASSWORD`, `GOVC_DATACENTER`, `GOVC_DATASTORE`, `GOVC_URL`, `GOVC_NETWORK`, `GOVC_RESOURCE_POOL`, and `GOVC_FOLDER` need to be populated.

```bash
export CONTROL_PLANE_ENDPOINT_IP=
export MGMT_CLUSTER_KUBECONFIG_PATH=

export CLUSTER_NAME="vmware-k8s-124"
export OUTPUT_FILE="${CLUSTER_NAME}-migration.yaml"
export VARIANT="vmware-k8s-1.24"
export K8S_VERSION="1.24"
export AGENT_IMAGE_VERSION=$(cli --version | sed -e "s/^.* //g")
export SONOBUOY_TEST_AGENT_IMAGE_URI="public.ecr.aws/bottlerocket-test-system/sonobuoy-test-agent:v${AGENT_IMAGE_VERSION}"
export MIGRATION_TEST_AGENT_IMAGE_URI="public.ecr.aws/bottlerocket-test-system/migration-test-agent:v${AGENT_IMAGE_VERSION}"
export VSPHERE_K8S_CLUSTER_RESOURCE_AGENT_IMAGE_URI="public.ecr.aws/bottlerocket-test-system/vsphere-k8s-cluster-resource-agent:v${AGENT_IMAGE_VERSION}"
export VSPHERE_VM_RESOURCE_AGENT_IMAGE_URI="public.ecr.aws/bottlerocket-test-system/vsphere-vm-resource-agent:v${AGENT_IMAGE_VERSION}"
export ASSUME_ROLE="~"
export REGION="us-west-2"
export UPGRADE_VERSION="v1.11.1"
export STARTING_VERSION="v1.11.0"
export METADATA_URL="https://updates.bottlerocket.aws/2020-07-07/${VARIANT}/x86_64"
export TARGETS_URL="https://updates.bottlerocket.aws/targets"
export OVA_NAME="bottlerocket-${VARIANT}-x86_64-${STARTING_VERSION}.ova"
export MGMT_CLUSTER_KUBECONFIG_BASE64=$(cat ${MGMT_CLUSTER_KUBECONFIG_PATH} | base64)

cli add-secret map  \
 --name "vsphere-creds" \
 "username=${GOVC_USERNAME}" \
 "password=${GOVC_PASSWORD}"

cat eks/vmware-migration-test.yaml \
  | sed -e "s/<CLUSTER-NAME>/$(echo ${CLUSTER_NAME} | sed 's/[^a-zA-Z 0-9]/\\&/g')/g" \
  | sed -e "s/<SONOBUOY-TEST-AGENT-IMAGE-URI>/$(echo ${SONOBUOY_TEST_AGENT_IMAGE_URI} | sed 's/[^a-zA-Z 0-9]/\\&/g')/g" \
  | sed -e "s/<ASSUME-ROLE>/$(echo ${ASSUME_ROLE} | sed 's/[^a-zA-Z 0-9]/\\&/g')/g" \
  | sed -e "s/<REGION>/$(echo ${REGION} | sed 's/[^a-zA-Z 0-9]/\\&/g')/g" \
  | sed -e "s/<UPGRADE-VERSION>/$(echo ${UPGRADE_VERSION} | sed 's/[^a-zA-Z 0-9]/\\&/g')/g" \
  | sed -e "s/<STARTING-VERSION>/$(echo ${STARTING_VERSION} | sed 's/[^a-zA-Z 0-9]/\\&/g')/g" \
  | sed -e "s/<METADATA-URL>/$(echo ${METADATA_URL} | sed 's/[^a-zA-Z 0-9]/\\&/g')/g" \
  | sed -e "s/<TARGETS-URL>/$(echo ${TARGETS_URL} | sed 's/[^a-zA-Z 0-9]/\\&/g')/g" \
  | sed -e "s/<MIGRATION-TEST-AGENT-IMAGE-URI>/$(echo ${MIGRATION_TEST_AGENT_IMAGE_URI} | sed 's/[^a-zA-Z 0-9]/\\&/g')/g" \
  | sed -e "s/<VSPHERE-K8S-CLUSTER-RESOURCE-AGENT-IMAGE-URI>/$(echo ${VSPHERE_K8S_CLUSTER_RESOURCE_AGENT_IMAGE_URI} | sed 's/[^a-zA-Z 0-9]/\\&/g')/g" \
  | sed -e "s/<VSPHERE-VM-RESOURCE-AGENT-IMAGE-URI>/$(echo ${VSPHERE_VM_RESOURCE_AGENT_IMAGE_URI} | sed 's/[^a-zA-Z 0-9]/\\&/g')/g" \
  | sed -e "s/<OVA-NAME>/$(echo ${OVA_NAME} | sed 's/[^a-zA-Z 0-9]/\\&/g')/g" \
  | sed -e "s/<K8S-VERSION>/$(echo ${K8S_VERSION} | sed 's/[^a-zA-Z 0-9]/\\&/g')/g" \
  | sed -e "s/<CONTROL-PLANE-ENDPOINT-IP>/$(echo ${CONTROL_PLANE_ENDPOINT_IP} | sed 's/[^a-zA-Z 0-9]/\\&/g')/g" \
  | sed -e "s/<MGMT-CLUSTER-KUBECONFIG-BASE64>/$(echo ${MGMT_CLUSTER_KUBECONFIG_BASE64} | sed 's/[^a-zA-Z 0-9]/\\&/g')/g" \
  | sed -e "s/<VCENTER-DATACENTER>/$(echo ${GOVC_DATACENTER} | sed 's/[^a-zA-Z 0-9]/\\&/g')/g" \
  | sed -e "s/<VCENTER-DATASTORE>/$(echo ${GOVC_DATASTORE} | sed 's/[^a-zA-Z 0-9]/\\&/g')/g" \
  | sed -e "s/<VCENTER-HOST-URL>/$(echo ${GOVC_URL} | sed 's/[^a-zA-Z 0-9]/\\&/g')/g" \
  | sed -e "s/<VCENTER-NETWORK>/$(echo ${GOVC_NETWORK} | sed 's/[^a-zA-Z 0-9]/\\&/g')/g" \
  | sed -e "s/<VCENTER-RESOURCE-POOL>/$(echo ${GOVC_RESOURCE_POOL} | sed 's/[^a-zA-Z 0-9]/\\&/g')/g" \
  | sed -e "s/<VCENTER-WORKLOAD-FOLDER>/$(echo ${GOVC_FOLDER} | sed 's/[^a-zA-Z 0-9]/\\&/g')/g" \
  > ${OUTPUT_FILE}
```

### Conformance Testing on `vmware-k8s` Variants

This codeblock assumes that your vSphere config file has been sourced. Specifically, the variables `GOVC_USERNAME`, `GOVC_PASSWORD`, `GOVC_DATACENTER`, `GOVC_DATASTORE`, `GOVC_URL`, `GOVC_NETWORK`, `GOVC_RESOURCE_POOL`, and `GOVC_FOLDER` need to be populated.

```bash
export CONTROL_PLANE_ENDPOINT_IP=
export MGMT_CLUSTER_KUBECONFIG_PATH=

export CLUSTER_NAME="vmware-k8s-124"
export OUTPUT_FILE="${CLUSTER_NAME}.yaml"
export VARIANT="vmware-k8s-1.24"
export K8S_VERSION="1.24"
export AGENT_IMAGE_VERSION=$(cli --version | sed -e "s/^.* //g")
export SONOBUOY_TEST_AGENT_IMAGE_URI="public.ecr.aws/bottlerocket-test-system/sonobuoy-test-agent:v${AGENT_IMAGE_VERSION}"
export VSPHERE_K8S_CLUSTER_RESOURCE_AGENT_IMAGE_URI="public.ecr.aws/bottlerocket-test-system/vsphere-k8s-cluster-resource-agent:v${AGENT_IMAGE_VERSION}"
export VSPHERE_VM_RESOURCE_AGENT_IMAGE_URI="public.ecr.aws/bottlerocket-test-system/vsphere-vm-resource-agent:v${AGENT_IMAGE_VERSION}"
export ASSUME_ROLE="~"
export REGION="us-west-2"
export SONOBUOY_MODE="quick"
export VERSION="v1.11.1"
export METADATA_URL="https://updates.bottlerocket.aws/2020-07-07/${VARIANT}/x86_64"
export TARGETS_URL="https://updates.bottlerocket.aws/targets"
export OVA_NAME="bottlerocket-${VARIANT}-x86_64-${VERSION}.ova"
export MGMT_CLUSTER_KUBECONFIG_BASE64=$(cat $MGMT-CLUSTER-KUBECONFIG-PATH | base64)

cli add-secret map  \
 --name "vsphere-creds" \
 "username=${GOVC_USERNAME}" \
 "password=${GOVC_PASSWORD}"

cat eks/vmware-sonobuoy-test.yaml \
  | sed -e "s/<CLUSTER-NAME>/$(echo ${CLUSTER_NAME} | sed 's/[^a-zA-Z 0-9]/\\&/g')/g" \
  | sed -e "s/<SONOBUOY-TEST-AGENT-IMAGE-URI>/$(echo ${SONOBUOY_TEST_AGENT_IMAGE_URI} | sed 's/[^a-zA-Z 0-9]/\\&/g')/g" \
  | sed -e "s/<ASSUME-ROLE>/$(echo ${ASSUME_ROLE} | sed 's/[^a-zA-Z 0-9]/\\&/g')/g" \
  | sed -e "s/<REGION>/$(echo ${REGION} | sed 's/[^a-zA-Z 0-9]/\\&/g')/g" \
  | sed -e "s/<METADATA-URL>/$(echo ${METADATA_URL} | sed 's/[^a-zA-Z 0-9]/\\&/g')/g" \
  | sed -e "s/<TARGETS-URL>/$(echo ${TARGETS_URL} | sed 's/[^a-zA-Z 0-9]/\\&/g')/g" \
  | sed -e "s/<SONOBUOY-MODE>/$(echo ${SONOBUOY_MODE} | sed 's/[^a-zA-Z 0-9]/\\&/g')/g" \
  | sed -e "s/<VSPHERE-K8S-CLUSTER-RESOURCE-AGENT-IMAGE-URI>/$(echo ${VSPHERE_K8S_CLUSTER_RESOURCE_AGENT_IMAGE_URI} | sed 's/[^a-zA-Z 0-9]/\\&/g')/g" \
  | sed -e "s/<VSPHERE-VM-RESOURCE-AGENT-IMAGE-URI>/$(echo ${VSPHERE_VM_RESOURCE_AGENT_IMAGE_URI} | sed 's/[^a-zA-Z 0-9]/\\&/g')/g" \
  | sed -e "s/<OVA-NAME>/$(echo ${OVA_NAME} | sed 's/[^a-zA-Z 0-9]/\\&/g')/g" \
  | sed -e "s/<K8S-VERSION>/$(echo ${K8S_VERSION} | sed 's/[^a-zA-Z 0-9]/\\&/g')/g" \
  | sed -e "s/<CONTROL-PLANE-ENDPOINT-IP>/$(echo ${CONTROL_PLANE_ENDPOINT_IP} | sed 's/[^a-zA-Z 0-9]/\\&/g')/g" \
  | sed -e "s/<MGMT-CLUSTER-KUBECONFIG-BASE64>/$(echo ${MGMT_CLUSTER_KUBECONFIG_BASE64} | sed 's/[^a-zA-Z 0-9]/\\&/g')/g" \
  | sed -e "s/<VCENTER-DATACENTER>/$(echo ${GOVC_DATACENTER} | sed 's/[^a-zA-Z 0-9]/\\&/g')/g" \
  | sed -e "s/<VCENTER-DATASTORE>/$(echo ${GOVC_DATASTORE} | sed 's/[^a-zA-Z 0-9]/\\&/g')/g" \
  | sed -e "s/<VCENTER-HOST-URL>/$(echo ${GOVC_URL} | sed 's/[^a-zA-Z 0-9]/\\&/g')/g" \
  | sed -e "s/<VCENTER-NETWORK>/$(echo ${GOVC_NETWORK} | sed 's/[^a-zA-Z 0-9]/\\&/g')/g" \
  | sed -e "s/<VCENTER-RESOURCE-POOL>/$(echo ${GOVC_RESOURCE_POOL} | sed 's/[^a-zA-Z 0-9]/\\&/g')/g" \
  | sed -e "s/<VCENTER-WORKLOAD-FOLDER>/$(echo ${GOVC_FOLDER} | sed 's/[^a-zA-Z 0-9]/\\&/g')/g" \
  > ${OUTPUT_FILE}
```

## kind

The files in [kind](./kind) are meant to be run on a `kind` cluster. Directions on how to use a `kind` cluster with TestSys can be found in our [QUICKSTART](../../docs/QUICKSTART.md).

### Conformance Testing on `aws-ecs` Variants

```bash
export CLUSTER_NAME="x86-64-aws-ecs-1"
export OUTPUT_FILE="${CLUSTER_NAME}.yaml"
export VARIANT="aws-ecs-1"
export ARCHITECTURE="x86_64"
export AGENT_IMAGE_VERSION=$(cli --version | sed -e "s/^.* //g")
export ACCESS_KEY_ID=$(aws configure get aws_access_key_id)
export SECRET_ACCESS_KEY=$(aws configure get aws_secret_access_key)
export ECS_TEST_AGENT_IMAGE_URI="public.ecr.aws/bottlerocket-test-system/ecs-test-agent:v${AGENT_IMAGE_VERSION}"
export ECS_RESOURCE_AGENT_IMAGE_URI="public.ecr.aws/bottlerocket-test-system/ecs-resource-agent:v${AGENT_IMAGE_VERSION}"
export EC2_RESOURCE_AGENT_IMAGE_URI="public.ecr.aws/bottlerocket-test-system/ec2-resource-agent:v${AGENT_IMAGE_VERSION}"
export ASSUME_ROLE="~"
export AWS_REGION="us-west-2"

export BOTTLEROCKET_AMI_ID=$(aws ssm get-parameter \
  --region ${AWS_REGION} \
  --name "/aws/service/bottlerocket/${VARIANT}/${ARCHITECTURE}/latest/image_id" \
  --query Parameter.Value --output text)

cli add-secret map  \
 --name "aws-creds" \
 "ACCESS_KEY_ID=${ACCESS_KEY_ID}" \
 "SECRET_ACCESS_KEY=${SECRET_ACCESS_KEY}"

cat kind/ecs-test.yaml \
  | sed -e "s/<CLUSTER-NAME>/$(echo ${CLUSTER_NAME} | sed 's/[^a-zA-Z 0-9]/\\&/g')/g" \
  | sed -e "s/<ECS-TEST-AGENT-IMAGE-URI>/$(echo ${ECS_TEST_AGENT_IMAGE_URI} | sed 's/[^a-zA-Z 0-9]/\\&/g')/g" \
  | sed -e "s/<ASSUME-ROLE>/$(echo ${ASSUME_ROLE} | sed 's/[^a-zA-Z 0-9]/\\&/g')/g" \
  | sed -e "s/<AWS-REGION>/$(echo ${AWS_REGION} | sed 's/[^a-zA-Z 0-9]/\\&/g')/g" \
  | sed -e "s/<ECS-RESOURCE-AGENT-IMAGE-URI>/$(echo ${ECS_RESOURCE_AGENT_IMAGE_URI} | sed 's/[^a-zA-Z 0-9]/\\&/g')/g" \
  | sed -e "s/<EC2-RESOURCE-AGENT-IMAGE-URI>/$(echo ${EC2_RESOURCE_AGENT_IMAGE_URI} | sed 's/[^a-zA-Z 0-9]/\\&/g')/g" \
  | sed -e "s/<BOTTLEROCKET-AMI-ID>/${BOTTLEROCKET_AMI_ID}/g" \
  > ${OUTPUT_FILE}
```

### Conformance Testing on `aws-k8s` Variants

```bash
export CLUSTER_NAME="x86-64-aws-k8s-124"
export OUTPUT_FILE="${CLUSTER_NAME}.yaml"
export VARIANT="aws-k8s-1.24"
export ARCHITECTURE="x86_64"
export AGENT_IMAGE_VERSION=$(cli --version | sed -e "s/^.* //g")
export ACCESS_KEY_ID=$(aws configure get aws_access_key_id)
export SECRET_ACCESS_KEY=$(aws configure get aws_secret_access_key)
export SONOBUOY_TEST_AGENT_IMAGE_URI="public.ecr.aws/bottlerocket-test-system/sonobuoy-test-agent:v${AGENT_IMAGE_VERSION}"
export EKS_RESOURCE_AGENT_IMAGE_URI="public.ecr.aws/bottlerocket-test-system/eks-resource-agent:v${AGENT_IMAGE_VERSION}"
export EC2_RESOURCE_AGENT_IMAGE_URI="public.ecr.aws/bottlerocket-test-system/ec2-resource-agent:v${AGENT_IMAGE_VERSION}"
export ASSUME_ROLE="~"
export AWS_REGION="us-west-2"
export SONOBUOY_MODE="quick"

export BOTTLEROCKET_AMI_ID=$(aws ssm get-parameter \
  --region ${AWS_REGION} \
  --name "/aws/service/bottlerocket/${VARIANT}/${ARCHITECTURE}/latest/image_id" \
  --query Parameter.Value --output text)

cli add-secret map  \
 --name "aws-creds" \
 "ACCESS_KEY_ID=${ACCESS_KEY_ID}" \
 "SECRET_ACCESS_KEY=${SECRET_ACCESS_KEY}"

cat kind/sonobuoy-test.yaml \
  | sed -e "s/<CLUSTER-NAME>/$(echo ${CLUSTER_NAME} | sed 's/[^a-zA-Z 0-9]/\\&/g')/g" \
  | sed -e "s/<SONOBUOY-TEST-AGENT-IMAGE-URI>/$(echo ${SONOBUOY_TEST_AGENT_IMAGE_URI} | sed 's/[^a-zA-Z 0-9]/\\&/g')/g" \
  | sed -e "s/<ASSUME-ROLE>/$(echo ${ASSUME_ROLE} | sed 's/[^a-zA-Z 0-9]/\\&/g')/g" \
  | sed -e "s/<AWS-REGION>/$(echo ${AWS_REGION} | sed 's/[^a-zA-Z 0-9]/\\&/g')/g" \
  | sed -e "s/<SONOBUOY-MODE>/$(echo ${SONOBUOY_MODE} | sed 's/[^a-zA-Z 0-9]/\\&/g')/g" \
  | sed -e "s/<EKS-RESOURCE-AGENT-IMAGE-URI>/$(echo ${EKS_RESOURCE_AGENT_IMAGE_URI} | sed 's/[^a-zA-Z 0-9]/\\&/g')/g" \
  | sed -e "s/<EC2-RESOURCE-AGENT-IMAGE-URI>/$(echo ${EC2_RESOURCE_AGENT_IMAGE_URI} | sed 's/[^a-zA-Z 0-9]/\\&/g')/g" \
  | sed -e "s/<BOTTLEROCKET-AMI-ID>/${BOTTLEROCKET_AMI_ID}/g" \
  > ${OUTPUT_FILE}
```

### Conformance Testing on `vmware-k8s` Variants

This codeblock assumes that your vSphere config file has been sourced. Specifically, the variables `GOVC_USERNAME`, `GOVC_PASSWORD`, `GOVC_DATACENTER`, `GOVC_DATASTORE`, `GOVC_URL`, `GOVC_NETWORK`, `GOVC_RESOURCE_POOL`, and `GOVC_FOLDER` need to be populated.

```bash
export CONTROL_PLANE_ENDPOINT_IP=
export MGMT_CLUSTER_KUBECONFIG_PATH=

export CLUSTER_NAME="vmware-k8s-124"
export OUTPUT_FILE="${CLUSTER_NAME}.yaml"
export VARIANT="vmware-k8s-1.24"
export K8S_VERSION="1.24"
export AGENT_IMAGE_VERSION=$(cli --version | sed -e "s/^.* //g")
export SONOBUOY_TEST_AGENT_IMAGE_URI="public.ecr.aws/bottlerocket-test-system/sonobuoy-test-agent:v${AGENT_IMAGE_VERSION}"
export VSPHERE_K8S_CLUSTER_RESOURCE_AGENT_IMAGE_URI="public.ecr.aws/bottlerocket-test-system/vsphere-k8s-cluster-resource-agent:v${AGENT_IMAGE_VERSION}"
export VSPHERE_VM_RESOURCE_AGENT_IMAGE_URI="public.ecr.aws/bottlerocket-test-system/vsphere-vm-resource-agent:v${AGENT_IMAGE_VERSION}"
export ASSUME_ROLE="~"
export REGION="us-west-2"
export SONOBUOY_MODE="quick"
export VERSION="v1.11.1"
export METADATA_URL="https://updates.bottlerocket.aws/2020-07-07/${VARIANT}/x86_64"
export TARGETS_URL="https://updates.bottlerocket.aws/targets"
export OVA_NAME="bottlerocket-${VARIANT}-x86_64-${VERSION}.ova"
export MGMT_CLUSTER_KUBECONFIG_BASE64=$(cat ${MGMT_CLUSTER_KUBECONFIG_PATH} | base64)

cli add-secret map  \
 --name "vsphere-creds" \
 "username=$GOVC_USERNAME" \
 "password=$GOVC_PASSWORD"

cat kind/vmware-sonobuoy-test.yaml \
  | sed -e "s/<CLUSTER-NAME>/$(echo ${CLUSTER_NAME} | sed 's/[^a-zA-Z 0-9]/\\&/g')/g" \
  | sed -e "s/<SONOBUOY-TEST-AGENT-IMAGE-URI>/$(echo ${SONOBUOY_TEST_AGENT_IMAGE_URI} | sed 's/[^a-zA-Z 0-9]/\\&/g')/g" \
  | sed -e "s/<ASSUME-ROLE>/$(echo ${ASSUME_ROLE} | sed 's/[^a-zA-Z 0-9]/\\&/g')/g" \
  | sed -e "s/<REGION>/$(echo ${REGION} | sed 's/[^a-zA-Z 0-9]/\\&/g')/g" \
  | sed -e "s/<METADATA-URL>/$(echo ${METADATA_URL} | sed 's/[^a-zA-Z 0-9]/\\&/g')/g" \
  | sed -e "s/<TARGETS-URL>/$(echo ${TARGETS_URL} | sed 's/[^a-zA-Z 0-9]/\\&/g')/g" \
  | sed -e "s/<SONOBUOY-MODE>/$(echo ${SONOBUOY_MODE} | sed 's/[^a-zA-Z 0-9]/\\&/g')/g" \
  | sed -e "s/<VSPHERE-K8S-CLUSTER-RESOURCE-AGENT-IMAGE-URI>/$(echo ${VSPHERE_K8S_CLUSTER_RESOURCE_AGENT_IMAGE_URI} | sed 's/[^a-zA-Z 0-9]/\\&/g')/g" \
  | sed -e "s/<VSPHERE-VM-RESOURCE-AGENT-IMAGE-URI>/$(echo ${VSPHERE_VM_RESOURCE_AGENT_IMAGE_URI} | sed 's/[^a-zA-Z 0-9]/\\&/g')/g" \
  | sed -e "s/<OVA-NAME>/$(echo ${OVA_NAME} | sed 's/[^a-zA-Z 0-9]/\\&/g')/g" \
  | sed -e "s/<K8S-VERSION>/$(echo ${K8S_VERSION} | sed 's/[^a-zA-Z 0-9]/\\&/g')/g" \
  | sed -e "s/<CONTROL-PLANE-ENDPOINT-IP>/$(echo ${CONTROL_PLANE_ENDPOINT_IP} | sed 's/[^a-zA-Z 0-9]/\\&/g')/g" \
  | sed -e "s/<MGMT-CLUSTER-KUBECONFIG-BASE64>/$(echo ${MGMT_CLUSTER_KUBECONFIG_BASE64} | sed 's/[^a-zA-Z 0-9]/\\&/g')/g" \
  | sed -e "s/<VCENTER-DATACENTER>/$(echo ${GOVC_DATACENTER} | sed 's/[^a-zA-Z 0-9]/\\&/g')/g" \
  | sed -e "s/<VCENTER-DATASTORE>/$(echo ${GOVC_DATASTORE} | sed 's/[^a-zA-Z 0-9]/\\&/g')/g" \
  | sed -e "s/<VCENTER-HOST-URL>/$(echo ${GOVC_URL} | sed 's/[^a-zA-Z 0-9]/\\&/g')/g" \
  | sed -e "s/<VCENTER-NETWORK>/$(echo ${GOVC_NETWORK} | sed 's/[^a-zA-Z 0-9]/\\&/g')/g" \
  | sed -e "s/<VCENTER-RESOURCE-POOL>/$(echo ${GOVC_RESOURCE_POOL} | sed 's/[^a-zA-Z 0-9]/\\&/g')/g" \
  | sed -e "s/<VCENTER-WORKLOAD-FOLDER>/$(echo ${GOVC_FOLDER} | sed 's/[^a-zA-Z 0-9]/\\&/g')/g" \
  > ${OUTPUT_FILE}
```
