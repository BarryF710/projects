package kMeans;

import java.io.IOException;
import java.util.*;
import java.util.List;
import org.apache.hadoop.conf.Configuration;
import org.apache.hadoop.conf.Configured;
import org.apache.hadoop.fs.FileSystem;
import org.apache.hadoop.fs.Path;
import org.apache.hadoop.io.IntWritable;
import org.apache.hadoop.io.LongWritable;
import org.apache.hadoop.io.SequenceFile;
import org.apache.hadoop.io.Text;
import org.apache.hadoop.mapreduce.Job;
import org.apache.hadoop.mapreduce.Mapper;
import org.apache.hadoop.mapreduce.Reducer;
import org.apache.hadoop.mapreduce.lib.input.FileInputFormat;
import org.apache.hadoop.mapreduce.lib.output.FileOutputFormat;
import org.apache.hadoop.util.Tool;
import org.apache.hadoop.util.ToolRunner;

public class KMeans extends Configured implements Tool {
    // Maximum iterations number of 20 is used
    private final static int maxIterations = 20;
    // Change the K value to 3 or 6
    private final static int K = 3;

    // A class that denotes a point with x and y coordinates
    public static class Point implements Comparable<Point> {
        private final double x;
        private final double y;

        // Constructor to initialize point from string
        public Point(String s) {
            String[] strings = s.split(",");
            this.x = Double.parseDouble(strings[0]);
            this.y = Double.parseDouble(strings[1]);
        }

        // Constructor to initialize point with x and y coordinates
        public Point(double x, double y) {
            this.x = x;
            this.y = y;
        }

        // Method for x coordinate
        public double getX() {
            return this.x;
        }

        // Method for y coordinate
        public double getY() {
            return this.y;
        }

        // Method that calculates the Euclidean distance between two points
        public static double calculateDistance(Point point1, Point point2) {
            double x_diff = point1.getX() - point2.getX();
            double y_diff = point1.getY() - point2.getY();
            return Math.sqrt(Math.pow(x_diff, 2) + Math.pow(y_diff, 2));
        }

        // Comparable interface implementation based on x and y coordinates
        @Override
        public int compareTo(Point o) {
            int compareX = Double.compare(this.getX(), o.getX());
            int compareY = Double.compare(this.getY(), o.getY());
            return compareX != 0 ? compareX : compareY;
        }

        // String representation of the point
        public String toString() {
            return this.x + "," + this.y;
        }

        // Static method to write points to a sequence file
        public static void writePointsToFile(List<Point> points, Configuration conf) throws IOException {
            Path centroid_path = new Path(conf.get("centroid.path"));
            FileSystem fs = FileSystem.get(conf);
            if (fs.exists(centroid_path)) {
                fs.delete(centroid_path, true);
            }

            try (SequenceFile.Writer centerWriter = SequenceFile.createWriter(conf,
                    SequenceFile.Writer.file(centroid_path),
                    SequenceFile.Writer.keyClass(Text.class),
                    SequenceFile.Writer.valueClass(IntWritable.class))) {

                final IntWritable value = new IntWritable(0);

                for (Point point : points) {
                    centerWriter.append(new Text(point.toString()), value);
                }
            }
        }
    }

    // Mapper Part
    public static class KMeansMapper extends Mapper<LongWritable, Text, Text, Text> {
        public List<Point> centers = new ArrayList<>();

        // Setup method to initialize centroids from sequence file
        @Override
        public void setup(Context context) throws IOException, InterruptedException {
            super.setup(context);
            ArrayList<Point> arrayList = new ArrayList<>();
            Configuration conf = context.getConfiguration();
            Path center_path = new Path(conf.get("centroid.path"));
            try (FileSystem fs = FileSystem.get(conf);
                 SequenceFile.Reader reader = new SequenceFile.Reader(conf, SequenceFile.Reader.file(center_path))) {

                Text key = new Text();
                IntWritable value = new IntWritable();
                while (reader.next(key, value)) {
                    arrayList.add(new Point(key.toString()));
                }
            }
            this.centers = arrayList;
        }

        // Map method to assign points to nearest centroid
        @Override
        public void map(LongWritable key, Text value, Context context) throws IOException, InterruptedException {
            Point point = new Point(value.toString());
            int index = -1;
            double minDistance = Double.MAX_VALUE;
            for (int i = 0; i < centers.size(); i++) {
                double distance = Point.calculateDistance(point, centers.get(i));
                if (distance < minDistance) {
                    minDistance = distance;
                    index = i;
                }
            }
            context.write(new Text(Integer.toString(index)), new Text(point.toString()));
        }

        @Override
        public void cleanup(Context context) throws IOException, InterruptedException {
        }
    }

    // Reducer Part
    public static class KMeansReducer extends Reducer<Text, Text, Text, Text> {
        public List<Point> new_centers = new ArrayList<>();

        // Enum to track convergence
        public enum Counter {
            CONVERGED
        }

        @Override
        public void setup(Context context) {
        }

        // Reduce method to calculate new centroids
        @Override
        public void reduce(Text key, Iterable<Text> values, Context context) throws IOException, InterruptedException {
            double sumX = 0d;
            double sumY = 0d;
            int count = 0;
            while (values.iterator().hasNext()) {
                count++;
                String line = values.iterator().next().toString();
                Point point = new Point(line);
                sumX = sumX + point.getX();
                sumY = sumY + point.getY();
            }

            double newX = sumX / count;
            double newY = sumY / count;
            Point center = new Point(newX + "," + newY);
            new_centers.add(center);
            String clusterID = "Cluster:" + key;

            context.write(new Text(clusterID), new Text(center.toString())); // key, new Text(center.toString())
        }

        // Cleanup method to write new centroids to file
        @Override
        public void cleanup(Context context) throws IOException, InterruptedException {
            super.setup(context);
            Configuration conf = context.getConfiguration();
            Point.writePointsToFile(this.new_centers, conf);
        }
    }

    // Method to generate initial centroids
    private static List<Point> initiateCenterPoints() {
        List<Point> list = new ArrayList<>();
        Random random = new Random();
        int min = 0;
        int max = 30;

        HashSet<Integer> set1 = new HashSet<>();
        HashSet<Integer> set2 = new HashSet<>();
        for (int i = 1; i <= KMeans.K; i++) {
            int rand1 = random.nextInt(max - min) + min;
            int rand2 = random.nextInt(max - min) + min;
//            while (set1.contains(rand1) || set2.contains(rand2)) {
//                rand1 = random.nextInt(max - min) + min;
//                rand2 = random.nextInt(max - min) + min;
//            }
            set1.add(rand1);
            set2.add(rand2);
            list.add(new Point(rand1, rand2));
        }
        return list;
    }

    // Main method to execute the KMeans algorithm
    public static void main(String[] args) throws Exception {
        Configuration conf = new Configuration();

        // Set centroid path
        Path center_path = new Path(args[2]);
        conf.set("centroid.path", center_path.toString());

        // Write initial centroids to file
        Point.writePointsToFile(initiateCenterPoints(), conf);

        long startTime = System.currentTimeMillis();

        // Iterate until convergence or max iterations
        int i = 1;
        while (i <= maxIterations) {
            ToolRunner.run(conf, new KMeans(), args);
            i++;
        }
        long endTime = System.currentTimeMillis();

        long duration = (endTime - startTime) / 1000;

        // Output total running time
        System.out.println("The total running time is " + duration + " seconds");
    }

    // Run method for executing the MapReduce job
    @Override
    public int run(String[] args) throws Exception {
        Configuration conf = getConf();
        Job job = Job.getInstance(conf, "KMeans");
        FileSystem fs = FileSystem.get(conf);

        // Configure job
        job.setJarByClass(KMeans.class);
        job.setMapperClass(KMeansMapper.class);
        job.setReducerClass(KMeansReducer.class);

        job.setOutputKeyClass(Text.class);
        job.setOutputValueClass(Text.class);
        job.setNumReduceTasks(1);

        // Set input and output paths
        FileInputFormat.addInputPath(job, new Path(args[0]));
        Path output = new Path(args[1]);
        fs.delete(output, true);
        FileOutputFormat.setOutputPath(job, output);

        // Execute job
        return job.waitForCompletion(true) ? 0 : 1;
    }
}



/*
 * for the following the args are
 * /Users/barrychen/Desktop/MIE1628_Assignments/MIE1628_A1/input_path/data_points.txt
 * /Users/barrychen/Desktop/MIE1628_Assignments/MIE1628_A1/output_path/kmeans/centroids/cen.seq
 * /Users/barrychen/Desktop/MIE1628_Assignments/MIE1628_A1/output_path/kmeans/final
 * 
 */
//import java.io.IOException;
//import java.util.Random;
//import org.apache.hadoop.conf.Configuration;
//import org.apache.hadoop.conf.Configured;
//import org.apache.hadoop.fs.FileSystem;
//import org.apache.hadoop.fs.Path;
//import org.apache.hadoop.io.IntWritable;
//import org.apache.hadoop.io.SequenceFile;
//import org.apache.hadoop.io.Text;
//import org.apache.hadoop.mapreduce.Job;
//import org.apache.hadoop.mapreduce.Mapper;
//import org.apache.hadoop.mapreduce.Reducer;
//import org.apache.hadoop.mapreduce.lib.input.FileInputFormat;
//import org.apache.hadoop.mapreduce.lib.output.FileOutputFormat;
//import org.apache.hadoop.mapreduce.lib.input.TextInputFormat;
//import org.apache.hadoop.mapreduce.lib.output.TextOutputFormat;
//import org.apache.hadoop.util.Tool;
//import org.apache.hadoop.util.ToolRunner;
//
//public class KMeans extends Configured implements Tool {
//
//    private static final int maxIters = 20; // number of maximum iterations
//    private static final int K = 3; // number of centroids
//    private static boolean centroidsInitialized = false; // check if centroids are initialized
//    private static double[][] centroids; // 2D array to store the centroids coordinates
//
//    // Mapper Part
//    public static class KMeansMapper extends Mapper<Object, Text, IntWritable, Text> {
//
//        private static final Random random = new Random();
//        private Path centerPath;
//
//        // Initialize centroids in mapper
//        public void setup(Context context) throws IOException, InterruptedException {
//            Configuration conf = context.getConfiguration();
//            centerPath = new Path(conf.get("centroid.path"));
//            if (!centroidsInitialized) {
//                if (isFirstIteration(centerPath)) {
//                    centroids = generateRandomCentroids(K);
//                    writeCentroidsToSequenceFile(centroids, centerPath);
//                } else {
//                    readCentroidsFromSequenceFile(centerPath);
//                }
//                centroidsInitialized = true;
//            }
//        }
//        
//        private boolean isFirstIteration(Path centerPath) throws IOException {
//            Configuration conf = new Configuration();
//            FileSystem fs = FileSystem.get(conf);
//            return !fs.exists(centerPath);
//        }
//        
//        private void writeCentroidsToSequenceFile(double[][] centroids, Path centerPath) throws IOException {
//            Configuration conf = new Configuration();
//            FileSystem fs = FileSystem.get(conf);
//
//            try (SequenceFile.Writer writer = SequenceFile.createWriter(fs, conf, centerPath, IntWritable.class, Text.class)) {
//                for (int i = 0; i < centroids.length; i++) {
//                    Text centroidText = new Text(centroids[i][0] + "," + centroids[i][1]);
//                    writer.append(new IntWritable(i), centroidText);
//                }
//            }
//        }
//
//        private void readCentroidsFromSequenceFile(Path centerPath) throws IOException {
//            Configuration conf = new Configuration();
//            FileSystem fs = FileSystem.get(conf);
//
//            try (SequenceFile.Reader reader = new SequenceFile.Reader(fs, centerPath, conf)) {
//                IntWritable key = new IntWritable();
//                Text value = new Text();
//                int idx = 0;
//
//                while (reader.next(key, value)) {
//                    String[] parts = value.toString().split(",");
//                    centroids[idx][0] = Double.parseDouble(parts[0]);
//                    centroids[idx][1] = Double.parseDouble(parts[1]);
//                    idx++;
//                }
//            }
//        }
//
//        // Function that assigns each point to its nearest centroid
//        public void map(Object key, Text value, Context context) throws IOException, InterruptedException {
//            String[] xy = value.toString().split(",");
//            double x = Double.parseDouble(xy[0]); // x coord
//            double y = Double.parseDouble(xy[1]); // y coord
//            double minDistance = Double.MAX_VALUE;
//            int index = 0; // index of the nearest centroid
//
//            for (int j = 0; j < centroids.length; j++) {
//                double cx = centroids[j][0]; // centroid x coord
//                double cy = centroids[j][1]; // centroid y coord
//
//                double distance = Math.sqrt(Math.pow(cx - x, 2) + Math.pow(cy - y, 2)); // calculate euclidean distance between the two points
//                if (distance < minDistance) {
//                    index = j;
//                    minDistance = distance;
//                }
//            }
//            context.write(new IntWritable(index), value); // write the assigned index (cluster number), the data point value
//        }
//
//        private double[][] generateRandomCentroids(int k) {
//            double[][] centroids = new double[k][2];
//            for (int i = 0; i < k; i++) {
//                centroids[i][0] = random.nextDouble() * 20;
//                centroids[i][1] = random.nextDouble() * 20;
//            }
//            return centroids; // generate k centroids using random number
//        }
//    }
//
//    // Reducer Part
//    public static class KMeansReducer extends Reducer<IntWritable, Text, Text, Text> {
//
//        private Path centerPath;
//
//        // Function that calculates new centroids
//        protected void setup(Context context) throws IOException, InterruptedException {
//            Configuration conf = context.getConfiguration();
//            centerPath = new Path(conf.get("centroid.path"));
//        }
//
//        protected void reduce(IntWritable key, Iterable<Text> values, Context context)
//                throws IOException, InterruptedException {
//            double mx = 0d; // new centroid x coord
//            double my = 0d; // new centroid y coord
//            int count = 0; // number of points with the same index
//
//            for (Text value : values) {
//                String[] xy = value.toString().split(",");
//                mx += Double.parseDouble(xy[0]); // sum all x coord values for points with same index
//                my += Double.parseDouble(xy[1]); // sum all y coord values for points with same index
//                count += 1; // total number of points for a index
//            }
//
//            mx = mx / count; // get the average x and y coord values
//            my = my / count;
//            String centroid = mx + "," + my; // new centroid
//            String clusterId = "Cluster:" + key;
//
//            context.write(new Text(clusterId), new Text(centroid));
//        }
//
//        protected void cleanup(Context context) throws IOException, InterruptedException {
//            writeCentroidsToSequenceFile(centroids, centerPath);
//        }
//
//        private void writeCentroidsToSequenceFile(double[][] centroids, Path centerPath) throws IOException {
//            Configuration conf = new Configuration();
//            FileSystem fs = FileSystem.get(conf);
//
//            try (SequenceFile.Writer writer = SequenceFile.createWriter(fs, conf, centerPath, IntWritable.class, Text.class)) {
//                for (int i = 0; i < centroids.length; i++) {
//                    Text centroidText = new Text(centroids[i][0] + "," + centroids[i][1]);
//                    writer.append(new IntWritable(i), centroidText);
//                }
//            }
//        }
//    }
//
//    public int run(String[] args) throws Exception {
//        Configuration conf = getConf();
//        FileSystem fs = FileSystem.get(conf);
//        Job job = new Job(conf);
//        job.setJarByClass(KMeans.class);
//
//        FileInputFormat.setInputPaths(job, args[0]);
//        Path outDir = new Path(args[2]);
//        fs.delete(outDir, true);
//        FileOutputFormat.setOutputPath(job, outDir);
//
//        job.setInputFormatClass(TextInputFormat.class);
//        job.setOutputFormatClass(TextOutputFormat.class);
//
//        job.setMapperClass(KMeansMapper.class);
//        job.setReducerClass(KMeansReducer.class);
//
//        job.setNumReduceTasks(1);
//
//        job.setMapOutputKeyClass(IntWritable.class);
//        job.setMapOutputValueClass(Text.class);
//        job.setOutputKeyClass(Text.class);
//        job.setOutputValueClass(Text.class);
//
//        return job.waitForCompletion(true) ? 0 : 1;
//    }
//
//    public static void main(String[] args) throws Exception {
//        Configuration conf = new Configuration();
//        FileSystem fs = FileSystem.get(conf);
//
//        Path centerPath = new Path(args[1]);
//        conf.set("centroid.path", centerPath.toString());
//
//        int iter = 1;
//        int checker = 1;
//        do {
//            checker ^= ToolRunner.run(conf, new KMeans(), args);
//            iter++;
//        } while (checker == 1 && iter < maxIters);
//    }
//}

/*
 * another version
 */
//public class KMeans extends Configured implements Tool {
//
//    private static final int maxIters = 20; // number of maximum iterations
//    private static final int K = 3; // number of centroids
//    private static boolean centroidsInitialized = false; // check if centroids are initialized
//    private static double[][] centroids; // 2D array to store the centroids coordinates
//
//    // Mapper Part
//    public static class KMeansMapper extends Mapper<Object, Text, IntWritable, Text> {
//
//        private static final Random random = new Random();
//        private Path centerPath;
//
//        // Initialize centroids in mapper
//        public void setup(Context context) throws IOException, InterruptedException {
//            Configuration conf = context.getConfiguration();
//            centerPath = new Path(conf.get("centroid.path"));
//            if (!centroidsInitialized) {
//                if (isFirstIteration(centerPath)) {
//                    centroids = generateRandomCentroids(K);
//                    writeCentroidsToSequenceFile(centroids, centerPath);
//                } else {
//                    readCentroidsFromSequenceFile(centerPath);
//                }
//                centroidsInitialized = true;
//            }
//        }
//        
//        private boolean isFirstIteration(Path centerPath) throws IOException {
//            Configuration conf = new Configuration();
//            FileSystem fs = FileSystem.get(conf);
//            return !fs.exists(centerPath);
//        }
//        
//        private void writeCentroidsToSequenceFile(double[][] centroids, Path centerPath) throws IOException {
//            Configuration conf = new Configuration();
//            FileSystem fs = FileSystem.get(conf);
//
//            try (SequenceFile.Writer writer = SequenceFile.createWriter(fs, conf, centerPath, IntWritable.class, Text.class)) {
//                for (int i = 0; i < centroids.length; i++) {
//                    Text centroidText = new Text(centroids[i][0] + "," + centroids[i][1]);
//                    writer.append(new IntWritable(i), centroidText);
//                }
//            }
//        }
//
//        private void readCentroidsFromSequenceFile(Path centerPath) throws IOException {
//            Configuration conf = new Configuration();
//            FileSystem fs = FileSystem.get(conf);
//
//            try (SequenceFile.Reader reader = new SequenceFile.Reader(fs, centerPath, conf)) {
//                IntWritable key = new IntWritable();
//                Text value = new Text();
//                int idx = 0;
//
//                while (reader.next(key, value)) {
//                    String[] parts = value.toString().split(",");
//                    centroids[idx][0] = Double.parseDouble(parts[0]);
//                    centroids[idx][1] = Double.parseDouble(parts[1]);
//                    idx++;
//                }
//            }
//        }
//
//        // Function that assigns each point to its nearest centroid
//        public void map(Object key, Text value, Context context) throws IOException, InterruptedException {
//            String[] xy = value.toString().split(",");
//            double x = Double.parseDouble(xy[0]); // x coord
//            double y = Double.parseDouble(xy[1]); // y coord
//            double minDistance = Double.MAX_VALUE;
//            int index = 0; // index of the nearest centroid
//
//            for (int j = 0; j < centroids.length; j++) {
//                double cx = centroids[j][0]; // centroid x coord
//                double cy = centroids[j][1]; // centroid y coord
//
//                double distance = Math.sqrt(Math.pow(cx - x, 2) + Math.pow(cy - y, 2)); // calculate euclidean distance between the two points
//                if (distance < minDistance) {
//                    index = j;
//                    minDistance = distance;
//                }
//            }
//            context.write(new IntWritable(index), value); // write the assigned index (cluster number), the data point value
//        }
//
//        private double[][] generateRandomCentroids(int k) {
//            double[][] centroids = new double[k][2];
//            for (int i = 0; i < k; i++) {
//                centroids[i][0] = random.nextDouble() * 20;
//                centroids[i][1] = random.nextDouble() * 20;
//            }
//            return centroids; // generate k centroids using random number
//        }
//    }
//
//    // Reducer Part
//    public static class KMeansReducer extends Reducer<IntWritable, Text, Text, Text> {
//
//        // Function that calculates new centroids
//        protected void reduce(IntWritable key, Iterable<Text> values, Context context)
//                throws IOException, InterruptedException {
//            double mx = 0d; // new centroid x coord
//            double my = 0d; // new centroid y coord
//            int count = 0; // number of points with the same index
//
//            for (Text value : values) {
//                String[] xy = value.toString().split(",");
//                mx += Double.parseDouble(xy[0]); // sum all x coord values for points with same index
//                my += Double.parseDouble(xy[1]); // sum all y coord values for points with same index
//                count += 1; // total number of points for a index
//            }
//
//            mx = mx / count; // get the average x and y coord values
//            my = my / count;
//            String centroid = mx + "," + my; // new centroid
//            String clusterId = "Cluster:" + key;
//
//            context.write(new Text(clusterId), new Text(centroid));
//        }
//    }
//
//    public int run(String[] args) throws Exception {
//        Configuration conf = getConf();
//        FileSystem fs = FileSystem.get(conf);
//        Job job = new Job(conf);
//        job.setJarByClass(KMeans.class);
//
//        FileInputFormat.setInputPaths(job, args[0]);
//        Path outDir = new Path(args[2]);
//        fs.delete(outDir, true);
//        FileOutputFormat.setOutputPath(job, outDir);
//
//        job.setInputFormatClass(TextInputFormat.class);
//        job.setOutputFormatClass(TextOutputFormat.class);
//
//        job.setMapperClass(KMeansMapper.class);
//        job.setReducerClass(KMeansReducer.class);
//
//        job.setNumReduceTasks(1);
//
//        job.setMapOutputKeyClass(IntWritable.class);
//        job.setMapOutputValueClass(Text.class);
//        job.setOutputKeyClass(Text.class);
//        job.setOutputValueClass(Text.class);
//
//        return job.waitForCompletion(true) ? 0 : 1;
//    }
//
//    public static void main(String[] args) throws Exception {
//        Configuration conf = new Configuration();
//        FileSystem fs = FileSystem.get(conf);
//
//        Path centerPath = new Path(args[1]);
//        conf.set("centroid.path", centerPath.toString());
//
//        int iter = 1;
//        int checker = 1;
//        do {
//            checker ^= ToolRunner.run(conf, new KMeans(), args);
//            iter++;
//        } while (checker == 1 && iter < maxIters);
//    }
//}

/* 
 * working version without sequence file
 * the args for this version should be:
 * /Users/barrychen/Desktop/MIE1628_Assignments/MIE1628_A1/input_path/data_points.txt
 * /Users/barrychen/Desktop/MIE1628_Assignments/MIE1628_A1/output_path/kmeans3/new_centroid
 * /Users/barrychen/Desktop/MIE1628_Assignments/MIE1628_A1/output_path/kmeans3/final
 */
//import java.io.IOException;
//import java.util.Random;
//import org.apache.hadoop.conf.Configuration;
//import org.apache.hadoop.conf.Configured;
//import org.apache.hadoop.fs.FileSystem;
//import org.apache.hadoop.fs.Path;
//import org.apache.hadoop.io.IntWritable;
//import org.apache.hadoop.io.Text;
//import org.apache.hadoop.mapreduce.Job;
//import org.apache.hadoop.mapreduce.Mapper;
//import org.apache.hadoop.mapreduce.Reducer;
//import org.apache.hadoop.mapreduce.lib.input.FileInputFormat;
//import org.apache.hadoop.mapreduce.lib.output.FileOutputFormat;
//import org.apache.hadoop.mapreduce.lib.input.TextInputFormat;
//import org.apache.hadoop.mapreduce.lib.output.TextOutputFormat;
//import org.apache.hadoop.util.Tool;
//import org.apache.hadoop.util.ToolRunner;
//
//public class KMeans extends Configured implements Tool {
//
//    private static final int maxIters = 20; // number of maximum iterations
//    private static final int K = 3; // number of centroids
//    private static boolean centroidsInitialized = false; // check if centroids are initialized
//    private static double[][] centroids; // 2D array to store the centroids coordinates
//
//    // Mapper Part
//    public static class KMeansMapper extends Mapper<Object, Text, IntWritable, Text> {
//
//        private static final Random random = new Random();
//
//        // Initialize centroids in mapper
//        public void setup(Context context) throws IOException, InterruptedException {
//            // Check if centroids are initialized
//        	if (!centroidsInitialized) {
//                centroids = generateRandomCentroids(K); // generate random centroids if not initialized
//                centroidsInitialized = true; // set to true once initialized
//            }
//        }
//
//        // Function that assign each point to its nearest centroid
//        public void map(Object key, Text value, Context context) throws IOException, InterruptedException {
//            String[] xy = value.toString().split(",");
//            double x = Double.parseDouble(xy[0]); // x coord
//            double y = Double.parseDouble(xy[1]); // y coord
//            double minDistance = Double.MAX_VALUE;
//            int index = 0; // index of the nearest centroid
//
//            for (int j = 0; j < centroids.length; j++) {
//                double cx = centroids[j][0]; // centroid x coord
//                double cy = centroids[j][1]; // centroid y coord
//
//                double distance = Math.sqrt(Math.pow(cx - x, 2) + Math.pow(cy - y, 2)); // calculate euclidean distance between the two points
//                if (distance < minDistance) {
//                    index = j;
//                    minDistance = distance;
//                }
//            }
//            context.write(new IntWritable(index), value); // write the assigned index (cluster number), the data point value
//        }
//
//        private double[][] generateRandomCentroids(int k) {
//            double[][] centroids = new double[k][2];
//            for (int i = 0; i < k; i++) {
//                centroids[i][0] = random.nextDouble() * 20;
//                centroids[i][1] = random.nextDouble() * 20;
//            }
//            return centroids; // generate k centroids using random number
//        }
//    }
//
//    // Reducer Part
//    public static class KMeansReducer extends Reducer<IntWritable, Text, Text, Text> {
//
//    	// Function that calculates new centroids
//        protected void reduce(IntWritable key, Iterable<Text> values, Context context)
//                throws IOException, InterruptedException {
//            double mx = 0d; // new centroid x coord
//            double my = 0d; // new centroid y coord 
//            int count = 0; // number of points with the same index
//
//            for (Text value : values) {
//                String[] xy = value.toString().split(",");
//                mx += Double.parseDouble(xy[0]); // sum all x coord values for points with same index
//                my += Double.parseDouble(xy[1]); // sum all y coord values for points with same index
//                count += 1; // total number of points for a index
//            }
//
//            mx = mx / count; // get the avarage x and y coord values
//            my = my / count;
//            String centroid = mx + "," + my; // new centroid
//            String clusterId = "Cluster:" + key;
//
//            context.write(new Text(clusterId), new Text(centroid));
//        }
//    }
//
//    public int run(String[] args) throws Exception {
//        Configuration conf = getConf();
//        FileSystem fs = FileSystem.get(conf);
//        Job job = new Job(conf);
//        job.setJarByClass(KMeans.class);
//
//        FileInputFormat.setInputPaths(job, args[0]);
//        Path outDir = new Path(args[1]);
//        fs.delete(outDir, true);
//        FileOutputFormat.setOutputPath(job, outDir);
//
//        job.setInputFormatClass(TextInputFormat.class);
//        job.setOutputFormatClass(TextOutputFormat.class);
//
//        job.setMapperClass(KMeansMapper.class);
//        job.setReducerClass(KMeansReducer.class);
//
//        job.setNumReduceTasks(1);
//
//        job.setMapOutputKeyClass(IntWritable.class);
//        job.setMapOutputValueClass(Text.class);
//        job.setOutputKeyClass(Text.class);
//        job.setOutputValueClass(Text.class);
//
//        return job.waitForCompletion(true) ? 0 : 1;
//    }
//
//    public static void main(String[] args) throws Exception {
//        Configuration conf = new Configuration();
//        FileSystem fs = FileSystem.get(conf);
//
//        int iter = 1;
//        int checker = 1;
//        do {
//            checker ^= ToolRunner.run(conf, new KMeans(), args);
//            iter++;
//        } while (checker == 1 && iter < maxIters);
//
//        // Final output
//        Job job = new Job(conf);
//        job.setJarByClass(KMeans.class);
//
//        FileInputFormat.setInputPaths(job, args[0]);
//        Path outDir = new Path(args[2]);
//        fs.delete(outDir, true);
//        FileOutputFormat.setOutputPath(job, outDir);
//
//        job.setInputFormatClass(TextInputFormat.class);
//        job.setOutputFormatClass(TextOutputFormat.class);
//
//        job.setMapperClass(KMeansMapper.class);
//        job.setReducerClass(KMeansReducer.class);
//
//        job.setNumReduceTasks(1);
//
//        job.setMapOutputKeyClass(IntWritable.class);
//        job.setMapOutputValueClass(Text.class);
//        job.setOutputKeyClass(Text.class);
//        job.setOutputValueClass(Text.class);
//
//        job.waitForCompletion(true);
//    }
//}


