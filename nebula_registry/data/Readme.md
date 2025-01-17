# Data Format for Nebula

This folder contains information about datasets and models for nebula. This is meant temporary and has the goal of prototyping an extension to the data package standard that is suitable for nebula and delta. 

Datasets and models meta description are based on the [Data Package Standard v2](https://datapackage.org/standard/data-package/).

The inbuild extensions [Table Dialect](https://datapackage.org/standard/table-dialect/) and [Table Schema](https://datapackage.org/standard/table-schema/) are used to describe the `iris.csv` and the `batches.meta.txt` in Cifar-10.

## Dataset

Additonally to the table extensions, we add an extension `delta` on both the data package level and the resource, i.e. file, level.

Datasets come in different forms, these are three examples with rising complexity:

1. [Iris]() - A CSV based dataset with features of flowers that is used to classify their species, uses internal files.
2. [Cifar-10](https://www.cs.toronto.edu/~kriz/cifar.html) - An image classification dataset that uses the IDX/UByte format 
3. [Coco](https://cocodataset.org/#home) - An image segmentation dataset. TODO

### The delta extension

On package level we add:

- `category`: classifcation | segmentation | annotation | LLM?
- `classes`: Number of classes to classify or segment
- `training|validation|test-count`: how many samples are in the different sets
- `input-shape` the underlying input shape
- `mirror` an url to use for the download (could be a forward at the beginning, but useful for counting)

On resource level we add:

- `origin`: Where do we get the data (remote, registry, or local-archive), the latter means it has been downloaded from a remote location or the registry
- `format`: A string identifying the type of loader and the parameters it needs.
- `local-storage`: Decides if the content stays after installation or is deleted. If we agree on a delta specific format we may convert other formats into that when installing it.

### Iris

The Iris dataset is old and was popular in classical ML. There are several sources with different formats:

- [UC Irvine Archive](https://archive.ics.uci.edu/dataset/53/iris) - Contains several (redudant?) data files.
- [Some GitHub Repo](https://gist.github.com/curran/a08a1080b88344b0c8a7) - Contains a CSV files too.

We best stick to a simple csv though.

See the [datapackage.json](./iris/datapackage.json)

License is CC-BY

### Cifar-10

Like MINST a popular dataset for image classification. 

See the [datapackage.json](./cifar10/datapackage.json)

REMARK: Move cifar-10-binary.tar.gz to the cifar-10 data folder and extract it - It will be ignored by git but not by docker. So it is on the filesystem of the docker deployment of `nebula_registry`.

License is similar to CC-BY

### Coco

An upcoming use case is image segmentation. The most promising dataset for tinkering is [coco](https://cocodataset.org/#home). However, we push this to the future, as the [data formats](https://cocodataset.org/#format-data) used for segmentation are more complicated and need more interconnectoin as we have descirbed for Iris and Cifar-10.

License is complicated as each image may have a different license.

## (Pretrained)-Models

Just for testing Mobilnetv3 for image classification: protobuf model format from [Kaggle](https://www.kaggle.com/models/google/mobilenet-v3/)