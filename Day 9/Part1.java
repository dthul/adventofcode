import java.io.IOException;
import java.nio.charset.StandardCharsets;
import java.nio.file.FileSystems;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.HashMap;
import java.util.HashSet;
import java.util.LinkedList;
import java.util.List;
import java.util.Map;
import java.util.Set;
import java.util.regex.Matcher;
import java.util.regex.Pattern;

public class Part1 {
	
	// Dirty hack but works well enough for now
	protected static int hash(String s1, String s2) {
		return s1.hashCode() + s2.hashCode();
	}
	
	// Calculates the total travel distance when visiting the cities in the given order
	protected static int totalDistance(List<String> cityNames, Map<Integer, Integer> map) {
		int totalDist = 0;
		for (int i = 0; i < cityNames.size() - 1; ++i) {
			totalDist += map.get(hash(cityNames.get(i), cityNames.get(i + 1)));
		}
		return totalDist;
	}
	
	// Naively generates all permutations of the given list
	protected static List<List<String>> permutations(List<String> l) {
		if (l.size() < 2)
			return Arrays.asList(l);
		List<List<String>> result = new LinkedList<List<String>>();
		for (int i = 0; i < l.size(); ++i) {
			List<String> subList = new LinkedList<String>(l);
			subList.remove(i);
			List<List<String>> subListPermutations = permutations(subList);
			for (List<String> permutation : subListPermutations) {
				permutation.add(0, l.get(i));
				result.add(permutation);
			}
		}
		return result;
	}
	
	public static void main(String[] args) {
		final Set<String> cityNames = new HashSet<String>();
		final Map<Integer, Integer> map = new HashMap<Integer, Integer>();
		final Pattern distance_regex = Pattern.compile("^(?<start>[a-zA-Z]+) to (?<end>[a-zA-Z]+) = (?<dist>[0-9]+)$");
		final Path path = FileSystems.getDefault().getPath("input.txt");
		List<String> distances;
		try {
			distances = Files.readAllLines(path, StandardCharsets.US_ASCII);
		} catch (IOException e) {
			System.out.println("Can't open input.");
			System.out.println(e.toString());
			return;
		}
		for (String distance : distances) {
			final Matcher m = distance_regex.matcher(distance);
			if (!m.matches())
				continue;
			final String start = m.group("start");
			final String end   = m.group("end");
			final int dist     = Integer.parseInt(m.group("dist"));
			cityNames.add(start);
			cityNames.add(end);
			map.put(hash(start, end), dist);
		}
		int minDist = Integer.MAX_VALUE;
		List<String> minPermutation = new ArrayList<String>();
		// Brute force all different permutations
		for (List<String> permutation : permutations(new ArrayList<String>(cityNames))) {
			int totalDist = totalDistance(permutation, map);
			if (totalDist < minDist) {
				minDist = totalDist;
				minPermutation = permutation;
			}
		}
		System.out.println("Shortest distance is " + minDist + " when traveling in the order " + minPermutation);
	}

}
